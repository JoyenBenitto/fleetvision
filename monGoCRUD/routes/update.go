package routes

import (
	"context"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	getcollection "github.com/sid-008/monGoCRUD/Collection"
	database "github.com/sid-008/monGoCRUD/databases"
	model "github.com/sid-008/monGoCRUD/model"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func UpdatePost(c *gin.Context) {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	var DB = database.ConnectDB()
	var postCollection = getcollection.GetCollection(DB, "Post")

	postId := c.Param("postId")
	var post model.Post

	defer cancel()

	objId, _ := primitive.ObjectIDFromHex(postId)

	if err := c.BindJSON(&post); err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"message": err})
		return
	}

	edited := bson.M{"title": post.Title, "article": post.Article}

	result, err := postCollection.UpdateOne(ctx, bson.M{"id": objId}, bson.M{"$set": edited})

	res := map[string]interface{}{"data": result}

	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"message": err})
		return
	}

	if result.MatchedCount < 1 {
		c.JSON(http.StatusInternalServerError, gin.H{"message": "Data doesn't exist"})
		return
	}

	c.JSON(http.StatusCreated, gin.H{"message": "data updated!", "Data": res})
}
