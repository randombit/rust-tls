
tls: tls.rc $(wildcard *.rs)
	rustc --test tls.rc
