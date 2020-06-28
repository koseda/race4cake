FROM gitpod/workspace-full-vnc:latest

USER gitpod

# Install softare required for development
RUN true "" \
	&& sudo apt-get update -q \
	&& sudo apt-get install -qy \
		libreoffice-calc \
		# Required for `libreoffice-calc`, you might be wondering why is it not provided by `libreoffice-calc`? Because ubuntu is maintained by monkeys
		libreoffice-java-common \
		# Also required, because now we are the downstream
		default-jre \
		# And it's not working even with those! Hug you ubuntu
	&& sudo apt-get autoremove -qy \
	&& sudo rm -rf /var/lib/apt/lists/*
