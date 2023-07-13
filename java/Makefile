java_run: lib
	javac lance/Lance.java lance/LanceDataset.java && RUST_BACKTRACE=full java -Djava.library.path=target/debug/ lance.Lance

.PHONY: lib

lib:
	cargo build

jheader:
	javac -h lance lance/Lance.java lance/LanceDataset.java

jlance_lib:
	cd lance && mvn compile
