import * as markdown from "./markdown.js";
import * as json from "./json.js";

export type Format = "markdown" | "json";

export function getFormatter(format: Format) {
  return format === "json" ? json : markdown;
}
