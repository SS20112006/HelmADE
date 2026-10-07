import { useState, useEffect, useCallback } from 'react';
import type { DiffSummary, DiffFile, FileDiffDetail, MergeResult, MergeStrategy } from '../types/diff';
import { gitDiffService } from '../services/gitDiffService';

export function useGitDiff(repoPath: string, swarmId: string, baseBranch?: string) {
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [summary, setSummary] = useState<DiffSummary | null>(null);

  const [selectedFile, setSelectedFile] = useState<DiffFile | null>(null);
  const [fileDetail, setFileDetail] = useState<FileDiffDetail | null>(null);
  const [loadingFile, setLoadingFile] = useState<boolean>(false);

  const [merging, setMerging] = useState<boolean>(false);
  const [mergeResult, setMergeResult] = useState<MergeResult | null>(null);

  const fetchDiff = useCallback(async () => {
    if (!repoPath || !swarmId) return;
    setLoading(true);
    setError(null);
    try {
      const res = await gitDiffService.getWorktreeDiff(repoPath, swarmId, baseBranch);
      setSummary(res);
      // Selecionar o primeiro ficheiro por padrão se nenhum estiver selecionado
      if (res.files.length > 0 && (!selectedFile || !res.files.some((f) => f.path === selectedFile.path))) {
        setSelectedFile(res.files[0]);
      } else if (res.files.length === 0) {
        setSelectedFile(null);
        setFileDetail(null);
      }
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : String(err);
      setError(msg);
    } finally {
      setLoading(false);
    }
  }, [repoPath, swarmId, baseBranch, selectedFile]);

  // Carregar patch detalhado quando o ficheiro selecionado mudar
  useEffect(() => {
    if (!selectedFile || !repoPath || !swarmId) {
      setFileDetail(null);
      return;
    }

    let isMounted = true;
    setLoadingFile(true);

    gitDiffService
      .getFileDiff(repoPath, swarmId, selectedFile.path, baseBranch)
      .then((detail) => {
        if (isMounted) {
          setFileDetail(detail);
        }
      })
      .catch((err: unknown) => {
        if (isMounted) {
          const msg = err instanceof Error ? err.message : String(err);
          setFileDetail({ path: selectedFile.path, patch: `Erro ao carregar patch: ${msg}` });
        }
      })
      .finally(() => {
        if (isMounted) setLoadingFile(false);
      });

    return () => {
      isMounted = false;
    };
  }, [selectedFile, repoPath, swarmId, baseBranch]);

  useEffect(() => {
    fetchDiff();
  }, [repoPath, swarmId, baseBranch]);

  const selectFile = useCallback((file: DiffFile) => {
    setSelectedFile(file);
  }, []);

  const executeMerge = useCallback(
    async (strategy: MergeStrategy, commitMessage?: string): Promise<MergeResult> => {
      setMerging(true);
      setError(null);
      try {
        const result = await gitDiffService.mergeWorktree(
          repoPath,
          swarmId,
          strategy,
          commitMessage,
          baseBranch
        );
        setMergeResult(result);
        return result;
      } catch (err: unknown) {
        const msg = err instanceof Error ? err.message : String(err);
        const failResult: MergeResult = {
          success: false,
          strategy,
          merged_branch: `helm/swarm-${swarmId}`,
          target_branch: baseBranch || 'main',
          commit_sha: null,
          conflicts: [msg],
          message: msg,
        };
        setMergeResult(failResult);
        setError(msg);
        return failResult;
      } finally {
        setMerging(false);
      }
    },
    [repoPath, swarmId, baseBranch]
  );

  const abortMerge = useCallback(async () => {
    try {
      await gitDiffService.abortMerge(repoPath);
      setMergeResult(null);
      setError(null);
      await fetchDiff();
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : String(err);
      setError(msg);
    }
  }, [repoPath, fetchDiff]);

  return {
    loading,
    error,
    summary,
    selectedFile,
    fileDetail,
    loadingFile,
    merging,
    mergeResult,
    selectFile,
    refresh: fetchDiff,
    executeMerge,
    abortMerge,
  };
}
