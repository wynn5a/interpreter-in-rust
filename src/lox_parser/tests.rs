    use super::*;
    use crate::token::Token;
    use crate::token_types::TokenType::RightParen;

    fn parse_and_print(tokens: Vec<Token>) -> (std::string::String, bool) {
        let mut parser = LoxParser::new(tokens);
        let expr = parser.parse_expression();
        let ast_printer = crate::expr::AstPrinter {};
        (expr.accept(&ast_printer), parser.has_error)
    }

    #[test]
    fn test_parser() {
        let tokens = vec![
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 1),
            Token::new(TokenType::Star, "*".to_string(), None, 1),
            Token::new(TokenType::Number, "3".to_string(), Some("3".to_string()), 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let (printed, has_error) = parse_and_print(tokens);
        assert_eq!(printed, "(+ 1 (* 2 3))");
        assert!(!has_error);
    }

    #[test]
    fn test_error() {
        // let tokens = vec![
        //     Token::new(LeftParen, "(".to_string(), Some("(".to_string()), 1),
        //     Token::new(Identifier, "foo".to_string(), Some("foo".to_string()), 1),
        //     Token::new(TokenType::Eof, "".to_string(), None, 1),
        // ];
        //
        // let mut parser = LoxParser::new(tokens);
        // let _ = parser.parse();
        // assert!(parser.has_error);
        //(92 +)
        let tokens = vec![
            Token::new(LeftParen, "(".to_string(), Some("(".to_string()), 1),
            Token::new(Number, "92".to_string(), Some("92.0".to_string()), 1),
            Token::new(Plus, "+".to_string(), None, 1),
            Token::new(RightParen, ")".to_string(), Some(")".to_string()), 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let _ = parser.parse();
        assert!(parser.has_error);
    }

    #[test]
    fn test_parser_precedence_chain() {
        let tokens = vec![
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 1),
            Token::new(TokenType::Star, "*".to_string(), None, 1),
            Token::new(TokenType::Number, "3".to_string(), Some("3".to_string()), 1),
            Token::new(TokenType::Minus, "-".to_string(), None, 1),
            Token::new(TokenType::Number, "4".to_string(), Some("4".to_string()), 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let (printed, has_error) = parse_and_print(tokens);
        assert_eq!(printed, "(- (+ 1 (* 2 3)) 4)");
        assert!(!has_error);
    }

    #[test]
    fn test_parser_grouping() {
        let tokens = vec![
            Token::new(LeftParen, "(".to_string(), Some("(".to_string()), 1),
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 1),
            Token::new(RightParen, ")".to_string(), Some(")".to_string()), 1),
            Token::new(TokenType::Star, "*".to_string(), None, 1),
            Token::new(TokenType::Number, "3".to_string(), Some("3".to_string()), 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let (printed, has_error) = parse_and_print(tokens);
        assert_eq!(printed, "(* (group (+ 1 2)) 3)");
        assert!(!has_error);
    }

    #[test]
    fn test_parser_unary_chain() {
        let tokens = vec![
            Token::new(TokenType::Bang, "!".to_string(), None, 1),
            Token::new(TokenType::Minus, "-".to_string(), None, 1),
            Token::new(TokenType::True, "true".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let (printed, has_error) = parse_and_print(tokens);
        assert_eq!(printed, "(! (- true))");
        assert!(!has_error);
    }

    #[test]
    fn test_parser_equality_comparison() {
        let tokens = vec![
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::Less, "<".to_string(), None, 1),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 1),
            Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
            Token::new(TokenType::False, "false".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let (printed, has_error) = parse_and_print(tokens);
        assert_eq!(printed, "(== (< 1 2) false)");
        assert!(!has_error);
    }

    #[test]
    fn test_parser_identifiers() {
        let tokens = vec![
            Token::new(TokenType::Identifier, "foo".to_string(), None, 1),
            Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
            Token::new(TokenType::Identifier, "bar".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let (printed, has_error) = parse_and_print(tokens);
        assert_eq!(printed, "(== foo bar)");
        assert!(!has_error);
    }

    // =========================================================================
    // STATEMENT PARSING TESTS
    // =========================================================================

    #[test]
    fn test_parse_print_statement() {
        let tokens = vec![
            Token::new(TokenType::Print, "print".to_string(), None, 1),
            Token::new(
                TokenType::Number,
                "42".to_string(),
                Some("42".to_string()),
                1,
            ),
            Token::new(Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert!(!parser.has_error);
        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::Print(print_stmt) => {
                // Verify it's a print statement with a literal 42
                match print_stmt.expression.as_ref() {
                    ExprEnum::Literal(_) => {
                        // Success - we have a print statement with a literal
                    }
                    _ => panic!("Expected literal expression in print statement"),
                }
            }
            _ => panic!("Expected print statement"),
        }
    }

    #[test]
    fn test_parse_expression_statement() {
        let tokens = vec![
            Token::new(
                TokenType::Number,
                "42".to_string(),
                Some("42".to_string()),
                1,
            ),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert!(!parser.has_error);
        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::Expression(expr_stmt) => {
                // Verify it's an expression statement with a binary expression
                match expr_stmt.expression.as_ref() {
                    ExprEnum::Binary(_) => {
                        // Success - we have an expression statement with binary expr
                    }
                    _ => panic!("Expected binary expression in expression statement"),
                }
            }
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_parse_multiple_statements() {
        let tokens = vec![
            // print 1;
            Token::new(TokenType::Print, "print".to_string(), None, 1),
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(Semicolon, ";".to_string(), None, 1),
            // print 2 + 3;
            Token::new(TokenType::Print, "print".to_string(), None, 2),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 2),
            Token::new(TokenType::Plus, "+".to_string(), None, 2),
            Token::new(TokenType::Number, "3".to_string(), Some("3".to_string()), 2),
            Token::new(Semicolon, ";".to_string(), None, 2),
            // 42;
            Token::new(
                TokenType::Number,
                "42".to_string(),
                Some("42".to_string()),
                3,
            ),
            Token::new(Semicolon, ";".to_string(), None, 3),
            Token::new(TokenType::Eof, "".to_string(), None, 3),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert!(!parser.has_error);
        assert_eq!(statements.len(), 3);

        // First statement should be print
        match &statements[0] {
            StmtEnum::Print(_) => {}
            _ => panic!("Expected first statement to be print"),
        }

        // Second statement should be print
        match &statements[1] {
            StmtEnum::Print(_) => {}
            _ => panic!("Expected second statement to be print"),
        }

        // Third statement should be expression
        match &statements[2] {
            StmtEnum::Expression(_) => {}
            _ => panic!("Expected third statement to be expression"),
        }
    }

    // =========================================================================
    // PHASE 4: PARSER - VARIABLE EXPRESSIONS TESTS
    // =========================================================================

    // -------------------------------------------------------------------------
    // Test 1: Parse simple variable expression
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_variable_expression() {
        // Tokens for: x
        let tokens = vec![
            Token::new(TokenType::Identifier, "x".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let expr = parser.parse_expression();

        // Should be a Variable expression, not a Literal
        match expr.as_ref() {
            ExprEnum::Variable(var) => {
                assert_eq!(var.name.lexeme, "x");
            }
            _ => panic!("Expected Variable expression"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 2: Parse variable in binary expression (a + 1)
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_variable_in_binary() {
        // Tokens for: a + 1
        let tokens = vec![
            Token::new(TokenType::Identifier, "a".to_string(), None, 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let expr = parser.parse_expression();

        // Should be a Binary expression with Variable on left
        match expr.as_ref() {
            ExprEnum::Binary(binary) => {
                // Left should be Variable
                match binary.left.as_ref() {
                    ExprEnum::Variable(var) => {
                        assert_eq!(var.name.lexeme, "a");
                    }
                    _ => panic!("Expected left to be Variable"),
                }

                // Operator should be Plus
                assert_eq!(binary.op.token_type, TokenType::Plus);

                // Right should be Literal number
                match binary.right.as_ref() {
                    ExprEnum::Literal(_) => {}
                    _ => panic!("Expected right to be Literal"),
                }
            }
            _ => panic!("Expected Binary expression"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 3: Parse two variables in binary expression (x + y)
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_two_variables_in_binary() {
        // Tokens for: x + y
        let tokens = vec![
            Token::new(TokenType::Identifier, "x".to_string(), None, 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Identifier, "y".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let expr = parser.parse_expression();

        // Should be a Binary expression with Variables on both sides
        match expr.as_ref() {
            ExprEnum::Binary(binary) => {
                // Left should be Variable 'x'
                match binary.left.as_ref() {
                    ExprEnum::Variable(var) => {
                        assert_eq!(var.name.lexeme, "x");
                    }
                    _ => panic!("Expected left to be Variable"),
                }

                // Right should be Variable 'y'
                match binary.right.as_ref() {
                    ExprEnum::Variable(var) => {
                        assert_eq!(var.name.lexeme, "y");
                    }
                    _ => panic!("Expected right to be Variable"),
                }
            }
            _ => panic!("Expected Binary expression"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 4: Parse variable in grouping expression
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_variable_in_grouping() {
        // Tokens for: (x)
        let tokens = vec![
            Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
            Token::new(TokenType::Identifier, "x".to_string(), None, 1),
            Token::new(TokenType::RightParen, ")".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let expr = parser.parse_expression();

        // Should be a Grouping containing Variable
        match expr.as_ref() {
            ExprEnum::Grouping(grouping) => match grouping.expression.as_ref() {
                ExprEnum::Variable(var) => {
                    assert_eq!(var.name.lexeme, "x");
                }
                _ => panic!("Expected inner expression to be Variable"),
            },
            _ => panic!("Expected Grouping expression"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 5: Parse variable in unary expression
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_variable_in_unary() {
        // Tokens for: -x
        let tokens = vec![
            Token::new(TokenType::Minus, "-".to_string(), None, 1),
            Token::new(TokenType::Identifier, "x".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let expr = parser.parse_expression();

        // Should be a Unary expression with Variable as operand
        match expr.as_ref() {
            ExprEnum::Unary(unary) => {
                assert_eq!(unary.op.token_type, TokenType::Minus);

                match unary.right.as_ref() {
                    ExprEnum::Variable(var) => {
                        assert_eq!(var.name.lexeme, "x");
                    }
                    _ => panic!("Expected operand to be Variable"),
                }
            }
            _ => panic!("Expected Unary expression"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 6: Parse complex expression with variables
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_complex_expression_with_variables() {
        // Tokens for: a * b + c
        let tokens = vec![
            Token::new(TokenType::Identifier, "a".to_string(), None, 1),
            Token::new(TokenType::Star, "*".to_string(), None, 1),
            Token::new(TokenType::Identifier, "b".to_string(), None, 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Identifier, "c".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let expr = parser.parse_expression();

        // Should be: (+ (* a b) c)
        match expr.as_ref() {
            ExprEnum::Binary(plus_expr) => {
                assert_eq!(plus_expr.op.token_type, TokenType::Plus);

                // Left should be (* a b)
                match plus_expr.left.as_ref() {
                    ExprEnum::Binary(star_expr) => {
                        assert_eq!(star_expr.op.token_type, TokenType::Star);

                        // Both operands should be Variables
                        match star_expr.left.as_ref() {
                            ExprEnum::Variable(var) => assert_eq!(var.name.lexeme, "a"),
                            _ => panic!("Expected 'a' to be Variable"),
                        }
                        match star_expr.right.as_ref() {
                            ExprEnum::Variable(var) => assert_eq!(var.name.lexeme, "b"),
                            _ => panic!("Expected 'b' to be Variable"),
                        }
                    }
                    _ => panic!("Expected left to be Binary expression"),
                }

                // Right should be Variable 'c'
                match plus_expr.right.as_ref() {
                    ExprEnum::Variable(var) => assert_eq!(var.name.lexeme, "c"),
                    _ => panic!("Expected 'c' to be Variable"),
                }
            }
            _ => panic!("Expected Binary expression"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 7: Parse different variable names
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_different_variable_names() {
        let test_cases = vec!["x", "myVar", "userName", "count123", "_private"];

        for name in test_cases {
            let tokens = vec![
                Token::new(TokenType::Identifier, name.to_string(), None, 1),
                Token::new(TokenType::Eof, "".to_string(), None, 1),
            ];

            let mut parser = LoxParser::new(tokens);
            let expr = parser.parse_expression();

            match expr.as_ref() {
                ExprEnum::Variable(var) => {
                    assert_eq!(var.name.lexeme, name);
                }
                _ => panic!("Expected Variable expression for '{}'", name),
            }
        }
    }

    // -------------------------------------------------------------------------
    // Test 8: Parse comparison with variables
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_comparison_with_variables() {
        // Tokens for: x > y
        let tokens = vec![
            Token::new(TokenType::Identifier, "x".to_string(), None, 1),
            Token::new(TokenType::Greater, ">".to_string(), None, 1),
            Token::new(TokenType::Identifier, "y".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let expr = parser.parse_expression();

        match expr.as_ref() {
            ExprEnum::Binary(binary) => {
                assert_eq!(binary.op.token_type, TokenType::Greater);

                // Both sides should be Variables
                match binary.left.as_ref() {
                    ExprEnum::Variable(var) => assert_eq!(var.name.lexeme, "x"),
                    _ => panic!("Expected left to be Variable"),
                }
                match binary.right.as_ref() {
                    ExprEnum::Variable(var) => assert_eq!(var.name.lexeme, "y"),
                    _ => panic!("Expected right to be Variable"),
                }
            }
            _ => panic!("Expected Binary expression"),
        }
    }

    // =========================================================================
    // PHASE 5: PARSER - VARIABLE DECLARATIONS TESTS
    // =========================================================================

    // -------------------------------------------------------------------------
    // Test 1: Parse var declaration with number initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_var_declaration_with_number() {
        // Tokens for: var x = 42;
        let tokens = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "x".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(
                TokenType::Number,
                "42".to_string(),
                Some("42".to_string()),
                1,
            ),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "x");
                assert!(var_stmt.initializer.is_some());
            }
            _ => panic!("Expected Var statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 2: Parse var declaration without initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_var_declaration_without_initializer() {
        // Tokens for: var y;
        let tokens = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "y".to_string(), None, 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "y");
                assert!(var_stmt.initializer.is_none());
            }
            _ => panic!("Expected Var statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 3: Parse var declaration with string initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_var_declaration_with_string() {
        // Tokens for: var name = "Alice";
        let tokens = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "name".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(
                TokenType::String,
                "\"Alice\"".to_string(),
                Some("Alice".to_string()),
                1,
            ),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "name");
                assert!(var_stmt.initializer.is_some());
            }
            _ => panic!("Expected Var statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 4: Parse var declaration with expression initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_var_declaration_with_expression() {
        // Tokens for: var sum = 1 + 2;
        let tokens = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "sum".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "sum");
                assert!(var_stmt.initializer.is_some());

                // Check that initializer is a Binary expression
                match var_stmt.initializer.as_ref().unwrap().as_ref() {
                    ExprEnum::Binary(_) => {}
                    _ => panic!("Expected Binary expression as initializer"),
                }
            }
            _ => panic!("Expected Var statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 5: Parse multiple var declarations
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_multiple_var_declarations() {
        // Tokens for:
        // var a = 1;
        // var b = 2;
        // var c;
        let tokens = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "a".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Var, "var".to_string(), None, 2),
            Token::new(TokenType::Identifier, "b".to_string(), None, 2),
            Token::new(TokenType::Equal, "=".to_string(), None, 2),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 2),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 2),
            Token::new(TokenType::Var, "var".to_string(), None, 3),
            Token::new(TokenType::Identifier, "c".to_string(), None, 3),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 3),
            Token::new(TokenType::Eof, "".to_string(), None, 3),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert_eq!(statements.len(), 3);

        // First var
        match &statements[0] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "a");
                assert!(var_stmt.initializer.is_some());
            }
            _ => panic!("Expected Var statement"),
        }

        // Second var
        match &statements[1] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "b");
                assert!(var_stmt.initializer.is_some());
            }
            _ => panic!("Expected Var statement"),
        }

        // Third var
        match &statements[2] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "c");
                assert!(var_stmt.initializer.is_none());
            }
            _ => panic!("Expected Var statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 6: Parse var declaration with variable initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_var_with_variable_initializer() {
        // Tokens for: var copy = original;
        let tokens = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "copy".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::Identifier, "original".to_string(), None, 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "copy");
                assert!(var_stmt.initializer.is_some());

                // Check that initializer is a Variable expression
                match var_stmt.initializer.as_ref().unwrap().as_ref() {
                    ExprEnum::Variable(var) => {
                        assert_eq!(var.name.lexeme, "original");
                    }
                    _ => panic!("Expected Variable expression as initializer"),
                }
            }
            _ => panic!("Expected Var statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 7: Parse mixed declarations and statements
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_mixed_var_and_statements() {
        // Tokens for:
        // var x = 10;
        // print x;
        // var y = 20;
        let tokens = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "x".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(
                TokenType::Number,
                "10".to_string(),
                Some("10".to_string()),
                1,
            ),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Print, "print".to_string(), None, 2),
            Token::new(TokenType::Identifier, "x".to_string(), None, 2),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 2),
            Token::new(TokenType::Var, "var".to_string(), None, 3),
            Token::new(TokenType::Identifier, "y".to_string(), None, 3),
            Token::new(TokenType::Equal, "=".to_string(), None, 3),
            Token::new(
                TokenType::Number,
                "20".to_string(),
                Some("20".to_string()),
                3,
            ),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 3),
            Token::new(TokenType::Eof, "".to_string(), None, 3),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert_eq!(statements.len(), 3);

        // First: var x = 10;
        match &statements[0] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "x");
            }
            _ => panic!("Expected Var statement"),
        }

        // Second: print x;
        match &statements[1] {
            StmtEnum::Print(_) => {}
            _ => panic!("Expected Print statement"),
        }

        // Third: var y = 20;
        match &statements[2] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "y");
            }
            _ => panic!("Expected Var statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 8: Parse var with boolean initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_var_with_boolean() {
        // Tokens for: var flag = true;
        let tokens = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "flag".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::True, "true".to_string(), None, 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "flag");
                assert!(var_stmt.initializer.is_some());
            }
            _ => panic!("Expected Var statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 9: Parse var with complex expression initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_var_with_complex_expression() {
        // Tokens for: var result = (1 + 2) * 3;
        let tokens = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "result".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 1),
            Token::new(TokenType::RightParen, ")".to_string(), None, 1),
            Token::new(TokenType::Star, "*".to_string(), None, 1),
            Token::new(TokenType::Number, "3".to_string(), Some("3".to_string()), 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "result");
                assert!(var_stmt.initializer.is_some());
            }
            _ => panic!("Expected Var statement"),
        }
    }

    #[test]
    fn test_parse_logical_or() {
        let tokens = vec![
            Token::new(TokenType::Nil, "nil".to_string(), None, 1),
            Token::new(TokenType::Or, "or".to_string(), None, 1),
            Token::new(
                TokenType::String,
                "\"ok\"".to_string(),
                Some("ok".to_string()),
                1,
            ),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let (printed, has_error) = parse_and_print(tokens);
        assert_eq!(printed, "(or nil ok)");
        assert!(!has_error);
    }

    #[test]
    fn test_parse_logical_and() {
        let tokens = vec![
            Token::new(TokenType::True, "true".to_string(), None, 1),
            Token::new(TokenType::And, "and".to_string(), None, 1),
            Token::new(TokenType::False, "false".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let (printed, has_error) = parse_and_print(tokens);
        assert_eq!(printed, "(and true false)");
        assert!(!has_error);
    }

    #[test]
    fn test_parse_logical_precedence_or_and() {
        // "true or false and nil" -> "(or true (and false nil))"
        let tokens = vec![
            Token::new(TokenType::True, "true".to_string(), None, 1),
            Token::new(TokenType::Or, "or".to_string(), None, 1),
            Token::new(TokenType::False, "false".to_string(), None, 1),
            Token::new(TokenType::And, "and".to_string(), None, 1),
            Token::new(TokenType::Nil, "nil".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let (printed, has_error) = parse_and_print(tokens);
        assert_eq!(printed, "(or true (and false nil))");
        assert!(!has_error);
    }

    #[test]
    fn test_parse_logical_precedence_equality() {
        // "1 == 1 or 2 == 2" -> "(or (== 1 1) (== 2 2))"
        let tokens = vec![
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::Or, "or".to_string(), None, 1),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 1),
            Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let (printed, has_error) = parse_and_print(tokens);
        assert_eq!(printed, "(or (== 1 1) (== 2 2))");
        assert!(!has_error);
    }

    #[test]
    fn test_parse_while_statement() {
        // while (true) print "loop";
        let tokens = vec![
            Token::new(TokenType::While, "while".to_string(), None, 1),
            Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
            Token::new(TokenType::True, "true".to_string(), None, 1),
            Token::new(TokenType::RightParen, ")".to_string(), None, 1),
            Token::new(TokenType::Print, "print".to_string(), None, 1),
            Token::new(
                TokenType::String,
                "\"loop\"".to_string(),
                Some("loop".to_string()),
                1,
            ),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert!(!parser.has_error);
        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::While(while_stmt) => {
                // Check condition
                match while_stmt.condition.as_ref() {
                    ExprEnum::Literal(l) => match &l.value {
                        LiteralValue::Boolean(b) => assert_eq!(*b, true),
                        _ => panic!("Expected boolean literal"),
                    },
                    _ => panic!("Expected Literal condition"),
                }

                // Check body
                match while_stmt.body.as_ref() {
                    StmtEnum::Print(_) => {}
                    _ => panic!("Expected Print statement body"),
                }
            }
            _ => panic!("Expected While statement"),
        }
    }

    #[test]
    fn test_parse_for_statement() {
        // for (var i = 0; i < 10; i = i + 1) print i;
        // Should desugar to:
        // {
        //   var i = 0;
        //   while (i < 10) {
        //     print i;
        //     i = i + 1;
        //   }
        // }
        let tokens = vec![
            Token::new(TokenType::For, "for".to_string(), None, 1),
            Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "i".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::Number, "0".to_string(), Some("0".to_string()), 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Identifier, "i".to_string(), None, 1),
            Token::new(TokenType::Less, "<".to_string(), None, 1),
            Token::new(
                TokenType::Number,
                "10".to_string(),
                Some("10".to_string()),
                1,
            ),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Identifier, "i".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::Identifier, "i".to_string(), None, 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::RightParen, ")".to_string(), None, 1),
            Token::new(TokenType::Print, "print".to_string(), None, 1),
            Token::new(TokenType::Identifier, "i".to_string(), None, 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        // This should pass if for loops are implemented correctly
        assert!(!parser.has_error);

        // Desugaring validation
        // The result should be a BlockStmt containing the initializer and a WhileStmt
        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::Block(block_stmt) => {
                // Should have 2 statements: initializer and while loop
                assert_eq!(block_stmt.statements.len(), 2);

                // 1. Initializer: var i = 0;
                match &block_stmt.statements[0] {
                    StmtEnum::Var(var_stmt) => {
                        assert_eq!(var_stmt.name.lexeme, "i");
                    }
                    _ => panic!("Expected Var statement as initializer"),
                }

                // 2. While loop
                match &block_stmt.statements[1] {
                    StmtEnum::While(while_stmt) => {
                        // Condition: i < 10
                        // Body should be a block containing the original body + increment
                        match while_stmt.body.as_ref() {
                            StmtEnum::Block(body_block) => {
                                assert_eq!(body_block.statements.len(), 2);
                                // Original body: print i;
                                match &body_block.statements[0] {
                                    StmtEnum::Print(_) => {}
                                    _ => panic!("Expected Print statement in loop body"),
                                }
                                // Increment: i = i + 1;
                                match &body_block.statements[1] {
                                    StmtEnum::Expression(_) => {}
                                    _ => panic!(
                                        "Expected Expression statement (increment) in loop body"
                                    ),
                                }
                            }
                            _ => panic!("Expected Block body for While loop (to hold increment)"),
                        }
                    }
                    _ => panic!("Expected While statement"),
                }
            }
            _ => panic!("Expected Block statement (outer scope for 'for' loop)"),
        }
    }

    #[test]
    fn test_parse_call_no_args() {
        // clock()
        let tokens = vec![
            Token::new(TokenType::Identifier, "clock".to_string(), None, 1),
            Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
            Token::new(TokenType::RightParen, ")".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let expr = parser.parse_expression();

        match expr.as_ref() {
            ExprEnum::Call(call) => {
                match call.callee.as_ref() {
                    ExprEnum::Variable(v) => assert_eq!(v.name.lexeme, "clock"),
                    _ => panic!("Expected variable callee"),
                }
                assert_eq!(call.arguments.len(), 0);
            }
            _ => panic!("Expected Call expression, got {:?}", expr),
        }
    }

    #[test]
    fn test_parse_call_with_args() {
        // add(1, 2)
        let tokens = vec![
            Token::new(TokenType::Identifier, "add".to_string(), None, 1),
            Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::Comma, ",".to_string(), None, 1),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 1),
            Token::new(TokenType::RightParen, ")".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let expr = parser.parse_expression();

        match expr.as_ref() {
            ExprEnum::Call(call) => {
                match call.callee.as_ref() {
                    ExprEnum::Variable(v) => assert_eq!(v.name.lexeme, "add"),
                    _ => panic!("Expected variable callee"),
                }
                assert_eq!(call.arguments.len(), 2);
            }
            _ => panic!("Expected Call expression, got {:?}", expr),
        }
    }

    // =========================================================================
    // PHASE 6: PARSER - FUNCTION DECLARATIONS TESTS
    // =========================================================================

    #[test]
    fn test_parse_function_no_args() {
        // fun foo() {}
        let tokens = vec![
            Token::new(TokenType::Fun, "fun".to_string(), None, 1),
            Token::new(TokenType::Identifier, "foo".to_string(), None, 1),
            Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
            Token::new(TokenType::RightParen, ")".to_string(), None, 1),
            Token::new(TokenType::LeftBrace, "{".to_string(), None, 1),
            Token::new(TokenType::RightBrace, "}".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert!(!parser.has_error);
        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::Function(func_stmt) => {
                assert_eq!(func_stmt.name.lexeme, "foo");
                assert_eq!(func_stmt.params.len(), 0);
                assert_eq!(func_stmt.body.len(), 0);
            }
            _ => panic!("Expected Function statement"),
        }
    }

    #[test]
    fn test_parse_function_with_args() {
        // fun sum(a, b) { print a + b; }
        let tokens = vec![
            Token::new(TokenType::Fun, "fun".to_string(), None, 1),
            Token::new(TokenType::Identifier, "sum".to_string(), None, 1),
            Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
            Token::new(TokenType::Identifier, "a".to_string(), None, 1),
            Token::new(TokenType::Comma, ",".to_string(), None, 1),
            Token::new(TokenType::Identifier, "b".to_string(), None, 1),
            Token::new(TokenType::RightParen, ")".to_string(), None, 1),
            Token::new(TokenType::LeftBrace, "{".to_string(), None, 1),
            Token::new(TokenType::Print, "print".to_string(), None, 1),
            Token::new(TokenType::Identifier, "a".to_string(), None, 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Identifier, "b".to_string(), None, 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::RightBrace, "}".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert!(!parser.has_error);
        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::Function(func_stmt) => {
                assert_eq!(func_stmt.name.lexeme, "sum");
                assert_eq!(func_stmt.params.len(), 2);
                assert_eq!(func_stmt.params[0].lexeme, "a");
                assert_eq!(func_stmt.params[1].lexeme, "b");
                assert_eq!(func_stmt.body.len(), 1);
            }
            _ => panic!("Expected Function statement"),
        }
    }
