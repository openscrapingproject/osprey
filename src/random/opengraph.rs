use opengraph;

// Determination: this lib is not worth it
fn main() {
    let x = r#"
<html prefix="og: http://ogp.me/ns#">
<head>
<title>The Rock (1996)</title>
<meta property="og:title" content="The Rock" />
<meta property="og:type" content="video.movie" />
<meta property="og:url" content="http://www.imdb.com/title/tt0117500/" />
<meta property="og:image" content="http://ia.media-imdb.com/images/rock.jpg" />
</head>
</html>
             "#;
    match opengraph::extract(&mut x.to_string().as_bytes(), Default::default()) {
        Ok(object) => {
            println!("{:?}", object);
        },
        Err(_) => println!("error occured"),
    }
}
