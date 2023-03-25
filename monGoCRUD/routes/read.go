package routes

import (
	"context"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	getCollection "github.com/sid-008/monGoCRUD/Collection"
	database "github.com/sid-008/monGoCRUD/databases"
	model "github.com/sid-008/monGoCRUD/model"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func ReadOnePost(c *gin.Context) {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	var DB = database.ConnectDB()
	var postCollection = getCollection.GetCollection(DB, "Posts")

	postId := c.Param("postId")
	var result model.Post

	defer cancel()

	objId, _ := primitive.ObjectIDFromHex(postId)

	err := postCollection.FindOne(ctx, bson.M{"id": objId}).Decode(&result)

	res := map[string]interface{}{"data": result}

	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"message": err})
		return
	}

	c.JSON(http.StatusCreated, gin.H{"message": "Success!", "Data": res})

}
