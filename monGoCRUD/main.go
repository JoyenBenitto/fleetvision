package main

import (
	"github.com/gin-gonic/gin"
	"github.com/sid-008/monGoCRUD/routes"
)

func main() {
	router := gin.Default()

	router.POST("/", routes.CreatePost)

	router.GET("getOne/:postId", routes.ReadOnePost)

	router.PUT("/update/:postId", routes.UpdatePost)

	router.DELETE("/delete/:postId", routes.DeletePost)

	router.Run("localhost: 3000")
}
