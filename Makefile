build:
	docker build -t button-led-blinks .
	docker run --rm -v ${CURDIR}/target:/app/target button-led-blinks
