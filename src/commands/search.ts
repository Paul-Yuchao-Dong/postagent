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

    const words = q.split(/\s+/).filter(Boolean);
    const scored = [...projects.values()]
      .map((p) => {
        const haystack = `${p.name} ${p.description} ${p.resources.map((r) => r.name).join(" ")} ${p.resources.flatMap((r) => r.actions.map((a) => a.name)).join(" ")}`.toLowerCase();
        const hits = words.filter((w) => haystack.includes(w)).length;
        return { project: p, hits };
      })
      .filter((s) => s.hits > 0)
      .sort((a, b) => b.hits - a.hits);
    const matched = scored.map((s) => s.project);

    if (matched.length === 0) {
      console.log("No projects found.");
      return;
    }

    console.log(formatter.formatProjectList(matched));
  });
