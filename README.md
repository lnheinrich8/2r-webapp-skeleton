# 2R Web Application Skeleton

## Overview

The 2R Web Application Skeleton is a starter kit for teams or solo developers who want the reliability of a typed, high-performance Rust backend without giving up the rapid iteration of a modern React frontend. This skeleton delivers all of the scafolding most basic web applications need so your first sprint can focus on business logic rather than plumbing.

## Server Functionality

The server is an Axum-based REST API with built in session handling. It is built to work with PostrgeSQL databases and uses r2d2 pooling with the Diesel ORM. The server follows a layered architechture, seperating concerns into a versioned API/transport layer, a service/business logic layer, and a data access layer. User authentication flows through bcrypt-backed login/register endpoints that issue JWT cookies, middleware enforces protected routes, custom exception types keep error responses consistent, and an emailer service sends verification links to complete registration.

## Client Functionality

The client is a Vite-powered React SPA. It uses the React router for navigation and a custom AuthContext provider that centralizes the user session state. The responsive and functional UI provides a collapsible sidebar to navigate between protected pages, log out of the application, and view application settings. The client follows feature based file/folder organization so further development can scale cleanly.

## Getting Set Up

1. **Clone and Install** - ```git clone```, then run ```npm install``` inside the client directory and run ```cargo build``` inside the server directory to fetch dependecies.
2. **Enviornment Variables** - Both the client and server directories should have their own ```.env``` files. See the section below for required enviornment variables.
3. **Database** - Currently this server skeleton does not support migrations and is configured to use only a ```users``` table in a PostgreSQL database. To support more tables you will have to define the corresponding Diesel models/schema entries and extend the repository layer.

## Client .env Requirements

Notes: 
- replace the parenthesis and everything inside with actual value that is described
- the client .env only contains one variable

Variables:

```VITE_API_BASE_URL```=(host where Axum server is running on)

## Server .env Requirements

Notes: 
- replace the parenthesis and everything inside with actual value that is described
- the server requires a valid email with the SMTP credentials in the .env. The ```EMAIL_PASS``` value must be an app-specific password from your SMTP provider (not your normal login password)

Variables:

```DATABASE_URL```=(postgres://user:password@host:port/database_name)

```JWT_SECRET```=(a secret string you make up used as a symmetric signing key for the user session token)

```JWT_EMAIL_SECRET```=(a secret string you make up used as a symmetric signing key for the registration verification token)

```EMAIL_USER```=(the actual email that you use on the server to send the verification emails e.g. bobbyjoe@gmail.com)

```EMAIL_PASS```=(the app password given to you by your SMTP provider. With gmail you can get an app password in google settings)

## UI Pictures

**Login page:**

<img width="1033" height="727" alt="2r_login" src="https://github.com/user-attachments/assets/7a385864-38d4-409a-9e20-21368dcedfca" />
<br/><br/>
<br/><br/>

**Main layout with sidebar collapsed:**

<img width="1152" height="858" alt="2r_mainlayout" src="https://github.com/user-attachments/assets/a56d52ba-381a-4356-87dc-ff26a2017569" />
<br/><br/>
<br/><br/>

**Main layout with sidebar expanded:**

<img width="1152" height="858" alt="2r_mainlayout_expanded" src="https://github.com/user-attachments/assets/c16f28ba-e11f-4720-a9ee-283d4cd88d09" />
<br/><br/>
<br/><br/>

**Settings modal:**

<img width="1258" height="921" alt="2r_settingsmodal" src="https://github.com/user-attachments/assets/5399ac15-d281-43c8-be78-f8276f9330a9" />
