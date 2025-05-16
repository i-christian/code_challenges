package router

import (
	"net/http"
	"slices"
)

// Router struct defines a simple router
type Router struct {
	*http.ServeMux
	globalChain []func(http.Handler) http.Handler
	routerChain []func(http.Handler) http.Handler
	isSubRouter bool
}

// NewRouter initialises a new router for the application
func NewRouter() *Router {
	return &Router{
		ServeMux: http.NewServeMux(),
	}
}

// Use method is used to add a single middleware to the application handler
func (r *Router) Use(mw func(http.Handler) http.Handler) {
	if r.isSubRouter {
		r.routerChain = append(r.routerChain, mw)
	} else {
		r.globalChain = append(r.globalChain, mw)
	}
}

// Group method groups related routes.
// Allows applying middleware to a specific group of routes.
func (r *Router) Group(fn func(r *Router)) {
	subRouter := &Router{
		routerChain: slices.Clone(r.routerChain),
		isSubRouter: true,
		ServeMux:    r.ServeMux,
	}

	fn(subRouter)
}

// HandleFunc registers handler function to a given pattern.
// Converts normal golang functions into http.Handler types
func (r *Router) HandleFunc(pattern string, h http.HandlerFunc) {
	r.Handle(pattern, h)
}

// Handle accepts application handlers.
// Appends middlerwares to the handler.
// It calls the ServeMux. Handle method which
// registers the handler for the given pattern.
func (r *Router) Handle(pattern string, h http.Handler) {
	for _, mw := range slices.Backward(r.routerChain) {
		h = mw(h)
	}

	r.ServeMux.Handle(pattern, h)
}

// ServeHTTP makes the router compatible with net/http
func (r *Router) ServeHTTP(w http.ResponseWriter, rq *http.Request) {
	var h http.Handler = r.ServeMux

	for _, mw := range slices.Backward(r.globalChain) {
		h = mw(h)
	}

	h.ServeHTTP(w, rq)
}
