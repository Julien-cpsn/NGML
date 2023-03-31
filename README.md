
# NGML

> Stands for : **N**ew **G**eneric **M**arkup **L**anguage

It is designed to simplify and renew HTML & XML (mainly android layouts).
Before being a standard it is now only a to-HTML compiling language.

Compile :
```bash
cargo build
```

Compile & run
```bash
cargo run
```

Usage :
```bash 
ngml <source_file.ngml> <output_file.html>
```

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
- lang: `2 chars`
- version: `string`

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
  - Example : `<text label="input_id"></text>` instead of `<label for="input_id"></label>`
- reverse
  - defines a label for several elements
  - Example : `<text reverse></text>` instead of `<bdo></bdo>`

### 4) a

`<a href=""></a>` becomes `<link ref=""></link>`

### 5) ul & ol

`<ul></ul>` becomes `<list></list>`

`<ol></ol>` becomes `<list ordered></list>`

### 6) Comment block

`<!-- Comment -->` becomes <* Comment *>

### 7) Img

`<img></img>` becomes `<image></image>`

### 8) Meter

`<meter></meter>` becomes `<gauge></gauge>`


## New tags

### 1) row

`<row></row>` is equivalent to `<div style="display: flex; flex-direction: row"></div>`

**Attributes :**
- reverse
  - Change row direction
  - Example : `<row reverse></row>`

### 2) column

`<column></column>` is equivalent to `<div style="display: flex; flex-direction: column"></div>`

**Attributes :**
- reverse
    - Change column direction
    - Example : `<column reverse></column>`

## Deleted tags

- `<bdo></bdo>`

## Styling

### 1) Line comment

Simply `// Comment line`

### 2) No more style attribute

Every CSS attribute is now a NGML tag attribute

Example:
```html
<text color="red"></text>
<div position="absolute"></div>
<div border="1px solid black"></div>
```

### 3) Selection possibilities

```html
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

`Document.id.someTagId` -> `<T>|undefined`

```html
<script>
/* Some javascript */
  
function example() {
    let someTag = Document.id.someTagId
}
</script>
```