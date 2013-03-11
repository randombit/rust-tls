
tls: tls.rc $(wildcard *.rs)
	rustc tls.rc
	rustc --test tls.rc

clean:
	rm -f tls libtls-*.so
