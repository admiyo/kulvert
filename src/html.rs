pub fn content() -> String {
let content = "
<!doctype html>
<html>
    <head>
        <meta charset=utf-8>
        <title>Forms</title>
    </head>
    <body>
        <h3>Will hit handle_post_1</h3>
        <hr>
        <h3>Will hit handle_post_2</h3>
        <hr>
        <h3>Will hit handle_post_3</h3>
    </body>
</html>";

    return content.to_string();
}
