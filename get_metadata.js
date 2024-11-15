const title = document.querySelector("meta[property='og:title']").content;
const cover = document.querySelector(".attr-cover").children[0].src;
const author =
  document.querySelectorAll(".text-muted")[1].parentElement.children[1]
    .children[0].textContent;
const artist =
  document.querySelectorAll(".text-muted")[2].parentElement.children[1]
    .children[0].textContent;
const description = document.querySelector(
  "meta[property='og:description']",
).content;
const chapter_ids = Array.from(document.querySelectorAll(".chapt")).map((x) =>
  Number(x.href.slice(x.href.length - 7)),
);

const json = JSON.stringify({
  title: {
    cover: cover,
    author: author,
    artist: artist,
    description: description,
    chapter_ids: chapter_ids,
  },
}).replace("title", title);

json;
