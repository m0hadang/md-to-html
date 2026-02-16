```yaml
version: '3'
services:
  # config service
  zookeeper: # service name
    # container image
    image: wurstmeister/zookeeper
    # container name
    container_name: zookeeper
    # config access port (container external:internal)
    ports:
      - "2181:2181"
  kafka:    
    image: wurstmeister/kafka 
    container_name: kafka
    ports:
      - "9092:9092"
    # config environment
    environment:
      KAFKA_ADVERTISED_HOST_NAME: localhost
      KAFKA_CREATE_TOPICS: "my-topic:3:1" # topic:partition:replication
      KAFKA_ZOOKEEPER_CONNECT: zookeeper:2181
    # config volume
    volumes:
      - /var/run/docker.sock
    # config dependency
    depends_on:
      - zookeeper
```