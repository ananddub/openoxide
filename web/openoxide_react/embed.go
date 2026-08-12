package web

import (
	"embed"
	"io/fs"

	"github.com/gofiber/fiber/v3"
	"github.com/gofiber/fiber/v3/middleware/static"
)

//go:embed all:dist
var assets embed.FS

// ServeSPA serves the embedded React SPA with client-side routing fallback.
func ServeSPA(app *fiber.App) {
	clientFS, _ := fs.Sub(assets, "dist")
	indexHTML, _ := fs.ReadFile(clientFS, "index.html")

	// Serve static files (JS, CSS, images, etc.)
	app.Use("/", static.New("", static.Config{
		FS:     clientFS,
		Browse: false,
	}))

	// SPA fallback: unmatched routes get index.html
	// so React Router handles client-side routing
	app.Use(func(c fiber.Ctx) error {
		c.Set("Content-Type", "text/html; charset=utf-8")
		return c.Send(indexHTML)
	})
}
