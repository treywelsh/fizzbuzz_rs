
# Containers related targets

buildc:
	docker build -t fizzbuzz ./

runc:
	docker run --name fizzbuzz_server --network host fizzbuzz

cleanc:
	docker rm fizzbuzz_server
	docker rmi fizzbuzz
#   rm -rf target
