import { Command } from "commander";
import { loadAllProjects } from "../loader/index.js";
import { getFormatter, type Format } from "../formatter/index.js";

export const searchCommand = new Command("search")
  .description("Search for projects by keyword")
  .argument("<query>", "Search query")
  .action((query: string, _opts: unknown, cmd: Command) => {
    const format = cmd.optsWithGlobals().format as Format;
    const formatter = getFormatter(format);
    const projects = loadAllProjects();
    const q = query.toLowerCase();

    const matched = [...projects.values()].filter((p) => {
      const haystack = `${p.name} ${p.description} ${p.resources.map((r) => r.name).join(" ")}`.toLowerCase();
      return q.split(/\s+/).every((word) => haystack.includes(word));
    });

    if (matched.length === 0) {
      console.log("No projects found.");
      return;
    }

    console.log(formatter.formatProjectList(matched));
  });
