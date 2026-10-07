use std::collections::HashMap;
use std::net::TcpListener;

pub const DEFAULT_MIN_PORT: u16 = 3001;
pub const DEFAULT_MAX_PORT: u16 = 3100;

#[derive(Debug, Clone)]
pub struct PortAllocator {
    min_port: u16,
    max_port: u16,
    /// Mapeia swarm_id para a porta alocada
    allocations: HashMap<String, u16>,
}

impl Default for PortAllocator {
    fn default() -> Self {
        Self::new(DEFAULT_MIN_PORT, DEFAULT_MAX_PORT)
    }
}

impl PortAllocator {
    pub fn new(min_port: u16, max_port: u16) -> Self {
        Self {
            min_port,
            max_port,
            allocations: HashMap::new(),
        }
    }

    /// Verifica se uma porta TCP está atualmente livre para bind no localhost
    pub fn is_port_available(port: u16) -> bool {
        match TcpListener::bind(("127.0.0.1", port)) {
            Ok(listener) => {
                // Fechar explicitamente o listener para liberar a porta de imediato
                drop(listener);
                true
            }
            Err(_) => false,
        }
    }

    /// Aloca uma porta livre para um swarm_id específico.
    /// Se o swarm já tiver uma porta alocada e ela ainda estiver livre, retorna a mesma.
    pub fn allocate(&mut self, swarm_id: &str) -> Result<u16, String> {
        if let Some(&existing_port) = self.allocations.get(swarm_id) {
            if Self::is_port_available(existing_port) {
                return Ok(existing_port);
            }
            // Se a porta previamente alocada foi ocupada externamente, remove a reserva
            self.allocations.remove(swarm_id);
        }

        // Conjunto de portas atualmente reservadas por outros swarms
        let reserved_ports: Vec<u16> = self.allocations.values().copied().collect();

        for candidate_port in self.min_port..=self.max_port {
            if !reserved_ports.contains(&candidate_port) && Self::is_port_available(candidate_port) {
                self.allocations.insert(swarm_id.to_string(), candidate_port);
                return Ok(candidate_port);
            }
        }

        Err(format!(
            "Não foi possível encontrar portas livres no intervalo {}-{}",
            self.min_port, self.max_port
        ))
    }

    /// Libera a porta previamente alocada para um swarm_id
    pub fn release(&mut self, swarm_id: &str) -> Option<u16> {
        self.allocations.remove(swarm_id)
    }

    /// Obtém a porta atualmente alocada para um swarm_id (se houver)
    pub fn get_port(&self, swarm_id: &str) -> Option<u16> {
        self.allocations.get(swarm_id).copied()
    }

    /// Lista todas as alocações ativas
    pub fn list_allocations(&self) -> HashMap<String, u16> {
        self.allocations.clone()
    }

    /// Gera o mapa de variáveis de ambiente com a porta injetada para a shell do PTY
    pub fn get_env_vars(&self, swarm_id: &str) -> HashMap<String, String> {
        let mut envs = HashMap::new();
        if let Some(port) = self.get_port(swarm_id) {
            let port_str = port.to_string();
            envs.insert("PORT".to_string(), port_str.clone());
            envs.insert("VITE_PORT".to_string(), port_str.clone());
            envs.insert("SERVER_PORT".to_string(), port_str.clone());
            envs.insert("DEV_SERVER_PORT".to_string(), port_str);
        }
        envs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_port_available_detects_free_ports() {
        // Encontrar porta dinâmica livre pelo SO usando porta 0
        let listener = TcpListener::bind("127.0.0.1:0").expect("falha ao criar socket teste");
        let free_port = listener.local_addr().unwrap().port();
        drop(listener);

        assert!(PortAllocator::is_port_available(free_port));
    }

    #[test]
    fn test_sequential_port_allocation_16_grids() {
        // Usar intervalo de teste para não colidir com portas de desenvolvimento locais
        let mut allocator = PortAllocator::new(14000, 14050);
        let mut allocated_ports = Vec::new();

        // Simular 16 grelhas paralelas requisitando portas
        for i in 1..=16 {
            let swarm_id = format!("grid-panel-{i}");
            let port = allocator.allocate(&swarm_id).expect("falha ao alocar porta");

            assert!(!allocated_ports.contains(&port), "porta {port} não deve colidir");
            allocated_ports.push(port);
        }

        assert_eq!(allocated_ports.len(), 16);
    }

    #[test]
    fn test_idempotent_allocation_returns_same_port() {
        let mut allocator = PortAllocator::new(15000, 15020);
        let swarm_id = "test-agent-alpha";

        let port_first = allocator.allocate(swarm_id).unwrap();
        let port_second = allocator.allocate(swarm_id).unwrap();

        assert_eq!(port_first, port_second);
    }

    #[test]
    fn test_release_and_reallocation() {
        let mut allocator = PortAllocator::new(16000, 16005);
        let swarm_1 = "swarm-1";
        let swarm_2 = "swarm-2";

        let port_1 = allocator.allocate(swarm_1).unwrap();
        assert_eq!(port_1, 16000);

        let port_2 = allocator.allocate(swarm_2).unwrap();
        assert_eq!(port_2, 16001);

        // Liberar swarm-1
        let released = allocator.release(swarm_1);
        assert_eq!(released, Some(16000));
        assert_eq!(allocator.get_port(swarm_1), None);

        // Próxima alocação deve poder reutilizar a porta 16000 liberada
        let swarm_3 = "swarm-3";
        let port_3 = allocator.allocate(swarm_3).unwrap();
        assert_eq!(port_3, 16000);
    }

    #[test]
    fn test_get_env_vars_generation() {
        let mut allocator = PortAllocator::new(17000, 17010);
        let swarm_id = "agent-frontend";

        let port = allocator.allocate(swarm_id).unwrap();
        let env_vars = allocator.get_env_vars(swarm_id);

        assert_eq!(env_vars.get("PORT").unwrap(), &port.to_string());
        assert_eq!(env_vars.get("VITE_PORT").unwrap(), &port.to_string());
        assert_eq!(env_vars.get("SERVER_PORT").unwrap(), &port.to_string());
        assert_eq!(env_vars.get("DEV_SERVER_PORT").unwrap(), &port.to_string());
    }
}
