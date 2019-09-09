default: dist

deps:
	go get github.com/fastly/terrctl/terrctl

dist:
	terrctl {src,assets}/**
