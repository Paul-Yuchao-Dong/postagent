import { createRequire } from "node:module";
import { Command } from "commander";
import { searchCommand } from "./commands/search.js";
import { getCommand } from "./commands/get.js";

const require = createRequire(import.meta.url);
const { version } = require("../package.json") as { version: string };

const program = new Command();

program
  .name("postagent")
  .description("CLI collection tool for agents")
  .version(version)
  .option("--format <type>", "Output format: markdown / json", "markdown");

program.addCommand(searchCommand);
program.addCommand(getCommand);

program.parse();
