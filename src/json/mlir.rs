pub mod ast;

pub mod error {
    #[derive(Debug, Clone)]
    pub struct NotFound {
        pub entity: String,
    }

    impl std::fmt::Display for NotFound {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "entity not found: {}", self.entity)
        }
    }

    impl std::error::Error for NotFound {}
} /* error */
