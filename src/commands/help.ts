import { Command } from "commander";

const API_BASE = "http://localhost:3000";

export const helpCommand = new Command("help")
  .description("Get project/resource/action details (progressive discovery)")
  .argument("[project]", "Project name")
  .argument("[resource]", "Resource name")
  .argument("[action]", "Action name")
  .action(async (projectName?: string, resourceName?: string, actionName?: string, _opts?: unknown, cmd?: Command) => {
    if (!projectName) {
      cmd?.parent?.help();
      return;
    }

    const format = (cmd ?? helpCommand).optsWithGlobals().format as string;

    const searchParams = new URLSearchParams({ project: projectName });
    if (resourceName) searchParams.set("resource", resourceName);
    if (actionName) searchParams.set("action", actionName);

    let data: Record<string, unknown>;
    try {
      const res = await fetch(`${API_BASE}/api/get?${searchParams}`);
      data = (await res.json()) as Record<string, unknown>;
      if (!res.ok) {
        console.error(data.error);
        if (data.available) {
          console.error(`Available: ${(data.available as string[]).join(", ")}`);
        }
        process.exitCode = 1;
        return;
      }
    } catch {
      console.error("Failed to connect to postagent server.");
      process.exitCode = 1;
      return;
    }

    if (format === "json") {
      console.log(JSON.stringify(data, null, 2));
      return;
    }

    if (!resourceName) {
      // Level 1: project → list resources
      const lines = [`${data.name}`, `  ${data.description}`, "", "Resources:"];
      for (const r of data.resources as { name: string; actions: string[] }[]) {
        lines.push(`  ${r.name}  (${r.actions.join(", ")})`);
      }
      console.log(lines.join("\n"));
    } else if (!actionName) {
      // Level 2: resource → list actions
      const lines = [`${projectName} > ${data.resource}`, "", "Actions:"];
      for (const a of data.actions as { name: string; method: string; path: string; summary: string }[]) {
        lines.push(`  ${a.name}  ${a.method} ${a.path}  ${a.summary}`);
      }
      console.log(lines.join("\n"));
    } else {
      // Level 3: action detail
      const lines = [
        `${data.project} > ${data.resource} > ${data.action}`,
        "",
        `  ${data.method} ${data.path}`,
        "",
        `  ${data.description}`,
      ];
      const actionParams = data.parameters as { name: string; in: string; type: string; required: boolean; description: string }[] | undefined;
      if (actionParams && actionParams.length > 0) {
        lines.push("", "Parameters:");
        for (const p of actionParams) {
          const req = p.required ? "required" : "optional";
          lines.push(`  --${p.name}  <${p.type}>  (${p.in}, ${req})  ${p.description}`);
        }
      }
      const body = data.requestBody as { contentType: string; schema: unknown } | null;
      if (body) {
        lines.push("", `Request Body (${body.contentType}):`);
        lines.push(JSON.stringify(body.schema, null, 2));
      }
      const responses = data.responses as { status: string; description: string }[] | undefined;
      if (responses && responses.length > 0) {
        lines.push("", "Responses:");
        for (const r of responses) {
          lines.push(`  ${r.status}  ${r.description}`);
        }
      }
      console.log(lines.join("\n"));
    }
  });
