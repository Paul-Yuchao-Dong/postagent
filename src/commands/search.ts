import { Command } from "commander";

const API_BASE = "http://localhost:3000";

export const searchCommand = new Command("search")
  .description("Search for projects by keyword")
  .argument("<query>", "Search query")
  .action(async (query: string, _opts: unknown, cmd: Command) => {
    const format = cmd.optsWithGlobals().format as string;

    let data: { name: string; description: string; resources: string[] }[];
    try {
      const res = await fetch(`${API_BASE}/api/search?q=${encodeURIComponent(query)}`);
      data = (await res.json()) as typeof data;
      if (!res.ok) {
        console.error((data as unknown as { error: string }).error);
        process.exitCode = 1;
        return;
      }
    } catch {
      console.error("Failed to connect to postagent server.");
      process.exitCode = 1;
      return;
    }

    if (data.length === 0) {
      console.log("No projects found.");
      return;
    }

    if (format === "json") {
      console.log(JSON.stringify(data, null, 2));
      return;
    }

    console.log(
      data
        .map((p) => `${p.name}\n  ${p.description}\n  Resources: ${p.resources.join(", ")}`)
        .join("\n\n"),
    );
  });
