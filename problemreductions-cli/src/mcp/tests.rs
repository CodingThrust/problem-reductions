#[cfg(test)]
mod tests {
    use crate::mcp::tools::McpServer;

    #[test]
    fn test_list_problems_returns_json() {
        let server = McpServer::new();
        let result = server.list_problems_inner();
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert!(json["num_types"].as_u64().unwrap() > 0);
        assert!(json["problems"].as_array().unwrap().len() > 0);
    }

    #[test]
    fn test_show_problem_known() {
        let server = McpServer::new();
        let result = server.show_problem_inner("MIS");
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["name"], "MaximumIndependentSet");
    }

    #[test]
    fn test_show_problem_unknown() {
        let server = McpServer::new();
        let result = server.show_problem_inner("NonExistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_find_path() {
        let server = McpServer::new();
        let result = server.find_path_inner("MIS", "QUBO", "minimize-steps", false);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert!(json["path"].as_array().unwrap().len() > 0);
    }

    #[test]
    fn test_find_path_all() {
        let server = McpServer::new();
        let result = server.find_path_inner("MIS", "QUBO", "minimize-steps", true);
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        // --all returns an array of path objects
        assert!(json.as_array().unwrap().len() > 0);
    }

    #[test]
    fn test_find_path_no_route() {
        let server = McpServer::new();
        // Pick two problems with no path (if any). Use an unknown problem to trigger an error.
        let result = server.find_path_inner("NonExistent", "QUBO", "minimize-steps", false);
        assert!(result.is_err());
    }

    #[test]
    fn test_neighbors_out() {
        let server = McpServer::new();
        let result = server.neighbors_inner("MIS", 1, "out");
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["direction"], "out");
        assert_eq!(json["hops"], 1);
    }

    #[test]
    fn test_neighbors_in() {
        let server = McpServer::new();
        let result = server.neighbors_inner("QUBO", 1, "in");
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["direction"], "in");
    }

    #[test]
    fn test_neighbors_both() {
        let server = McpServer::new();
        let result = server.neighbors_inner("MIS", 1, "both");
        assert!(result.is_ok());
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(json["direction"], "both");
    }

    #[test]
    fn test_neighbors_unknown_problem() {
        let server = McpServer::new();
        let result = server.neighbors_inner("NonExistent", 1, "out");
        assert!(result.is_err());
    }

    #[test]
    fn test_neighbors_invalid_direction() {
        let server = McpServer::new();
        let result = server.neighbors_inner("MIS", 1, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_export_graph() {
        let server = McpServer::new();
        let result = server.export_graph_inner();
        assert!(result.is_ok());
        // Verify it parses as valid JSON
        let json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert!(json.is_object());
    }
}
