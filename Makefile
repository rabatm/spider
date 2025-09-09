NAME = spider
CARGO = cargo

all:
	@$(CARGO) build --release
	@cp ./target/release/$(NAME) ./$(NAME)
	@echo "\033[32m[OK]\033[0m Le binaire '$(NAME)' a été compilé et copié à la racine."

clean:
	@$(CARGO) clean
	@echo "\033[33m[INFO]\033[0m Fichiers de compilation nettoyés."

fclean: clean
	@rm -f $(NAME)
	@echo "\033[33m[INFO]\033[0m Binaire final nettoyé."

re: fclean all

run: all
	@./$(NAME)

.PHONY: all clean fclean re run