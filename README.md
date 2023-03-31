
# NGML

*N*ew *G*eneric *M*arkup *L*anguage

## Tag changes

### 1) DOCTYPE

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

Values :
- format: web, A4, A3...
- encoding: UTF-8

### 2) head & meta

`<head></head>` becomes `<settings></settings>`
```html
<settings>
	
</settings>
```

`<meta />` has a tag for all of its names

Existing tags :
```html
<application-name></application-name>
<author></author>
<description></description>
<generator></generator>
<keywords></keywords>
<viewport></viewport>
```

**Example :**

```html
<settings>
    <title>NGML Example & Syntax</title>
    <description>This page presents NGML and it's specific syntax</description>
    <author>Julien Cpsn</author>
</settings>
```

### 3) p

`<p></p>` becomes `<text></text>`

**Attributes :**

- h1 to h6
  - change text size
  - Example : `<text h2></text>` instead of `<h2></h2>`
- label
  - change text direction
  - Example : `<text reverse></text>` instead of `<bdo></bdo>`
- reverse
  - defines a label for several elements
  - Example : `<text label="input_id"></text>` instead of `<label for="input_id"></label>`

### 4) a

`<a href=""></a>` becomes `<link ref=""></link>`

### 5) ul & ol

`<ul></ul>` becomes `<list></list>`

`<ol></ol>` becomes `<list ordered></list>`

### 6) Comment block

`<!-- Comment -->` becomes <* Comment *>

### 7) Meter

`<meter></meter>` becomes `<gauge></gauge>`


## New tags

### 1) row

`<row></row>` is equivalent to `<div style="display: flex; flex-direction: row"></div>`

**Attributes :**
- reverse
  - Change row direction
  - **Example :** `<row reverse></row>`

### 2) column

`<column></column>` is equivalent to `<div style="display: flex; flex-direction: column"></div>`

**Attributes :**
- reverse
    - Change column direction
    - **Example :** `<column reverse></column>`

## 3) Deleted tags

- `<bdo></bdo>`

## Styling

### 1) Line comment

Simply `// Comment`

### 2) Selection possibilities

```css
<style>
/* Some CSS */

/* Targets text that have the reverse attribute */
text::reverse {

}

/* Targets reversed columns */
column::reverse {

}

/* Targets reversed h1 text that has someClass */
text::h1::reverse.someClass {

}
</style>
```

## Scripting

`Document.id.someTagId -> <T>|undefined`

```js
<script>
function example() {
    // Some script
    let someTag = Document.id.someTagId
}
</script>
```