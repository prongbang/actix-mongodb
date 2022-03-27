run:
	cargo run src/main.rs

test_user_me:
	curl -X POST -H 'Content-Type: application/json' \
	  -d "{}" \
      -i 'http://127.0.0.1:8080/v1/device'
