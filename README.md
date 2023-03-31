
# NGML
	
## Tag changes

`<!DOCTYPE>`
becomes
```html
<document
	format="web"
	enconding="UTF-8"
	lang="fr"
	version="1.0"
>
</document>
```
- format: web, A4, ...
- - -

`<head></head>` becomes `<settings></settings>`
`<meta />` has a tag for all of its names
```html
<settings>
	<title>NGML Example & Syntax</title>
	<description>This page presents NGML and it's specific syntax</description>
	<author>Julien Cpsn</author>
</settings>
```
- - -

`<p></p>` becomes `<text></text>`
- can handle h1 to h6 to change text heigth like : `<text h2></text>` instead of `<h2></h2>`
- can handle reverse to change text direction like : `<text reverse></text>` instead of `<bdo></bdo>`

- - -

`<a href=""></a>` becomes `<link ref=""></link>`

- - -

`<ul></ul>` becomes `<list></list>`
`<ol></ol>` becomes `<list ordered></list>`

- - -

`<!-- Comment -->` becomes <* Comment *>

- - -

`<meter></meter>` becomes `<gauge></gauge>`


## New tags

`<row></row>` is equivalent to `<div style="display: flex; flex-direction: row"></div>`
- possibility to add reverse in the tag like : `<row reverse></row>`

`<column></column>` is equivalent to `<div style="display: flex; flex-direction: column"></div>`
- possibility to add reverse in the tag like : `<column reverse></column>`

## Deleted tags

`<bdo></bdo>`

# Styling

```css
<style>
/* Some CSS */
text::reverse {

}
</style>
```

# Scripting

```js
<script>
// Some script
Document.id.someTagId
</script>
```