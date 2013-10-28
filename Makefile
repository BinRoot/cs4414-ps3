all: zhtta

zhtta: 
	rustc gash.rs && rustc zhtta.rs

clean:
	rm -rf \#* *~ zhtta gash