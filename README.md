# 2R Web Application Skeleton

## Overview

The 2R Web Application Skeleton is a starter kit for teams or solo developers who want the reliability of a typed, high-performance Rust backend without giving up the rapid iteration of a modern React frontend. This skeleton delivers all of the scafolding most basic web applications need so your first sprint can focus on business logic rather than plumbing.

## Server Functionality

The server is an Axum-based REST API with built in session handling. It is built to work with PostrgeSQL databases and uses r2d2 pooling with the Diesel ORM. The server follows a layered architechture, seperating concerns into a versioned API/transport layer, a service/business logic layer, and a data access layer. User authentication flows through bcrypt-backed login/register endpoints that issue JWT cookies, middleware enforces protected routes, custom exception types keep error responses consistent, and an emailer service sends verification links to complete registration.

## Client Functionality

The client is a Vite-powered React SPA. It uses the React router for navigation and a custom AuthContext provider that centralizes the user session state. The responsive and functional UI provides a collapsible sidebar to navigate between protected pages, log out of the application, and view application settings. The client follows feature based file/folder organization so further development can scale cleanly.

