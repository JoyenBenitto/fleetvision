package getcollection

import "go.mongodb.org/mongo-driver/mongo"

func GetCollection(client *mongo.Client, collectionName string) *mongo.Collection {
	collection := client.Database("DB1").Collection("Post")
	return collection
}
