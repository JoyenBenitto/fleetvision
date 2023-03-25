package routes

import (
	"context"
	"log"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	getcollection "github.com/sid-008/monGoCRUD/Collection"
	database "github.com/sid-008/monGoCRUD/databases"
	model "github.com/sid-008/monGoCRUD/model"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func CreatePost(c *gin.Context) {
	var DB = database.ConnectDB()
	var postCollection = getcollection.GetCollection(DB, "Posts")

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	post := new(model.Post)
	defer cancel()

	if err := c.BindJSON(&post); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"message": err})
		log.Fatal(err)
		return
	}

	postPayload := model.Post{
		ID:      primitive.NewObjectID(),
		Title:   post.Title,
		Article: post.Article,
	}

	result, err := postCollection.InsertOne(ctx, postPayload)

	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"message": err})
		return
	}

	c.JSON(
		http.StatusCreated,
		gin.H{"message": "Posted Successfully", "Data": map[string]interface{}{"data": result}},
	)
}
