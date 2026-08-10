BINARY=fugu
BUILD_DIR=bin

build:
	mkdir -p $(BUILD_DIR)
	go build -o $(BUILD_DIR)/$(BINARY) ./cmd

build-all:
	mkdir -p $(BUILD_DIR)/linux $(BUILD_DIR)/macos $(BUILD_DIR)/windows

	GOOS=linux GOARCH=amd64 go build -o $(BUILD_DIR)/linux/$(BINARY) ./cmd
	GOOS=darwin GOARCH=amd64 go build -o $(BUILD_DIR)/macos/$(BINARY) ./cmd
	GOOS=windows GOARCH=amd64 go build -o $(BUILD_DIR)/windows/$(BINARY).exe ./cmd

run:
	go run ./cmd

test:
	go test ./... -v

fmt:
	go fmt ./...

clean:
	rm -rf $(BUILD_DIR)