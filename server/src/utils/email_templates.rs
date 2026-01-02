pub const REGISTER_VERIFICATION_HTML: &str = r#"
    <html>
        <head>
            <title>Email Verified</title>
            <style>
                body {
                    font-family: 'Roboto', sans-serif;
                    text-align: center;
                    margin-top: 50px;
                }
                .container {
                    max-width: 500px;
                    margin: auto;
                    padding: 20px;
                    border: 1px solid #ccc;
                    border-radius: 15px;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Email Verified</h1>
                <p>Your account has been successfully verified. You can now navigate back to the login page and log in.</p>
            </div>
        </body>
    </html>
"#;

pub const EMAIL_UPDATE_VERIFICATION_HTML: &str = r#"
    <html>
        <head>
            <title>Email Updated</title>
            <style>
                body {
                    font-family: 'Roboto', sans-serif;
                    text-align: center;
                    margin-top: 50px;
                }
                .container {
                    max-width: 500px;
                    margin: auto;
                    padding: 20px;
                    border: 1px solid #ccc;
                    border-radius: 15px;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Email Updated</h1>
                <p>Your email address has been successfully updated.</p>
            </div>
        </body>
    </html>
"#;
