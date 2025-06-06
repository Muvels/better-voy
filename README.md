<div align="center">
  <h1>better-Voy</h1>
  <strong>A better WASM vector similarity search engine written in Rust</strong>
</div>
<br/>

![voy: a vector similarity search engine in WebAssembly][demo]


- **Tiny**: 75KB gzipped, 69KB brotli.
- **Fast**: Create the best search experience for the users. Voy uses [k-d tree][k-d-tree] to index and provide fast search
- **Tree Shakable**: Optimize bundle size and enable asynchronous capabilities for modern Web API, such as [Web Workers](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers).
- **Resumable**: Generate portable embeddings index anywhere, anytime.
- **Worldwide**: Designed to deploy and run on CDN edge servers.

## Installation - legacy voy (will be replaced with better-voy later)

```bash
# with npm
npm i voy-search

# with Yarn
yarn add voy-search

# with pnpm
pnpm add voy-search
```

## APIs

### `class Voy`

The Voy class encapsulates an index and exposes all the public methods Voy has to offer.

```ts
class Voy {
  /**
   * By instantiating with a resource, Voy will construct the index. If the resource is
   * absent, it will construct an empty index. Calling Voy.index() later on will override
   * the empty index.
   * @param {Resource | undefined} resource
   */
  constructor(resource?: Resource);
  /**
   * Index given resource. Voy.index() is designed for the use case where a Voy instance
   * is instantiated without a resource. It will override the existing index. If you'd like
   * to keep the existing index, you can use Voy.add() to add your resource to the index.
   * @param {Resource} resource
   */
  index(resource: Resource): void;
  /**
   * Search top k results with given query embedding.
   * @param {Float32Array} query: Query Embedding
   * @param {number} k: Number of items in the search result
   * @returns {SearchResult}
   */
  search(query: Float32Array, k: number): SearchResult;
  /**
   * Add given resource to the index.
   * @param {Resource} resource
   */
  add(resource: Resource): void;
  /**
   * Remove given resource from the index.
   * @param {Resource} resource
   */
  remove(resource: Resource): void;
  /**
   * Remove all resources from the index.
   */
  clear(): void;
  /**
   * @returns {number}
   */
  size(): number;
  /**
   * Serialize a Voy instance.
   * @returns {string}
   */
  serialize(): string;
  /**
   * Deserialize a serialized index into a Voy instance.
   * @param {string} serialized_index
   * @returns {Voy}
   */
  static deserialize(serialized_index: string): Voy;
}

interface Resource {
  embeddings: Array<{
    id: string; // id of the resource
    title: string; // title of the resource
    url: string; // url to the resource
    embeddings: number[]; // embeddings of the resource
  }>;
}

interface SearchResult {
  neighbors: Array<{
    id: string; // id of the resource
    title: string; // title of the resource
    url: string; // url to the resource
  }>;
}
```

### Individual Functions

Besides the Voy class, Voy also exports all the instance methods as individual functions.

#### `index(resource: Resource): SerializedIndex`

It indexes the given resource and returns a serialized index.

**Parameters**

```ts
interface Resource {
  embeddings: Array<{
    id: string; // id of the resource
    title: string; // title of the resource
    url: string; // url to the resource
    embeddings: number[]; // embeddings of the resource
  }>;
}
```

**Return**

```ts
type SerializedIndex = string;
```

#### `search(index: SerializedIndex, query: Query, k: NumberOfResult): SearchResult`

It deserializes the given index and search for the `k` nearest neighbors of the query.

**Parameter**

```ts
type SerializedIndex = string;

type Query = Float32Array; // embeddings of the search query

type NumberOfResult = number; // K top results to return
```

**Return**

```ts
interface SearchResult {
  neighbors: Array<{
    id: string; // id of the resource
    title: string; // title of the resource
    url: string; // url to the resource
  }>;
}
```

#### `add(index: SerializedIndex, resource: Resource): SerializedIndex`

It adds resources to the index and returns an updated serialized index.

**Parameter**

```ts
type SerializedIndex = string;

interface Resource {
  embeddings: Array<{
    id: string; // id of the resource
    title: string; // title of the resource
    url: string; // url to the resource
    embeddings: number[]; // embeddings of the resource
  }>;
}
```

**Return**

```ts
type SerializedIndex = string;
```

#### `remove(index: SerializedIndex, resource: Resource): SerializedIndex`

It removes resources from the index and returns an updated serialized index.

**Parameter**

```ts
type SerializedIndex = string;

interface Resource {
  embeddings: Array<{
    id: string; // id of the resource
    title: string; // title of the resource
    url: string; // url to the resource
    embeddings: number[]; // embeddings of the resource
  }>;
}
```

**Return**

```ts
type SerializedIndex = string;
```

#### `clear(index: SerializedIndex): SerializedIndex`

It removes all items from the index and returns an empty serialized index.

**Parameter**

```ts
type SerializedIndex = string;
```

**Return**

```ts
type SerializedIndex = string;
```

#### `size(index: SerializedIndex): number;`

It returns the size of the index.

**Parameter**

```ts
type SerializedIndex = string;
```

## Usage

### With Transformers

As of now, voy relies on libraries like [`transformers.js`][transformers.js] and [`web-ai`][web-ai] to generate embeddings for text:



### Multiple Indexes

```js
import { TextModel } from "@visheratin/web-ai";

const { Voy } = await import("voy-search");
const phrases = [
  "That is a very happy Person",
  "That is a Happy Dog",
  "Today is a sunny day",
  "Sun flowers are blooming",
];
const model = await (await TextModel.create("gtr-t5-quant")).model;
const processed = await Promise.all(phrases.map((q) => model.process(q)));

const data = processed.map(({ result }, i) => ({
  id: String(i),
  title: phrases[i],
  url: `/path/${i}`,
  embeddings: result,
}));
const resourceA = { embeddings: data.slice(0, 2) };
const resourceB = { embeddings: data.slice(2) };

const indexA = new Voy(resourceA);
const indexB = new Voy(resourceB);
```

## License
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)


[demo]: ./voy.gif "voy demo"
[web-ai]: https://github.com/visheratin/web-ai
[k-d-tree]: https://en.wikipedia.org/wiki/K-d_tree
[transformers.js]: https://github.com/xenova/transformers.js
