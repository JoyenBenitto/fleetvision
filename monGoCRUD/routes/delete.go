package routes

import (
	"context"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	getcollection "github.com/sid-008/monGoCRUD/Collection"
	database "github.com/sid-008/monGoCRUD/databases"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func DeletePost(c *gin.Context) {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	var DB = database.ConnectDB()
	postId := c.Param("postId")

	var postCollection = getcollection.GetCollection(DB, "Posts")
	defer cancel()
	objId, _ := primitive.ObjectIDFromHex(postId)
	result, err := postCollection.DeleteOne(ctx, bson.M{"id": objId})
	res := map[string]interface{}{"data": result}

	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"message": err})
	}

	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"message": "No data to delete"})
	}

	c.JSON(http.StatusCreated, gin.H{"message": "Article deleted successfully", "Data": res})
}
