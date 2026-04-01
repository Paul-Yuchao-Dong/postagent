import type {
  ProjectInfo,
  ResourceInfo,
  ActionInfo,
} from "../loader/index.js";

export function formatProjectList(projects: ProjectInfo[]): string {
  return projects
    .map((p) => {
      const resourceNames = p.resources.map((r) => r.name).join(", ");
      return `# ${p.name}\n${p.description.trim()}\nResources: ${resourceNames}`;
    })
    .join("\n\n");
}

export function formatProject(project: ProjectInfo): string {
  const lines = [`# ${project.name}`, project.description.trim(), "", "## Resources", ""];
  for (const r of project.resources) {
    const actionNames = r.actions.map((a) => a.name).join(", ");
    lines.push(`- **${r.name}** — (${actionNames})`);
  }
  return lines.join("\n");
}

export function formatResource(project: ProjectInfo, resource: ResourceInfo): string {
  const lines = [`# ${project.name} > ${resource.name}`, "", "## Actions", ""];
  for (const a of resource.actions) {
    lines.push(`- **${a.name}** — ${a.method} ${a.path} — ${a.summary}`);
  }
  return lines.join("\n");
}

export function formatAction(
  project: ProjectInfo,
  resource: ResourceInfo,
  action: ActionInfo,
): string {
  const lines = [
    `# ${project.name} > ${resource.name} > ${action.name}`,
    "",
    `${action.method} ${action.path}`,
    "",
    action.description.trim(),
  ];

  if (action.parameters.length > 0) {
    lines.push("", "## Parameters", "");
    lines.push("| Name | In | Type | Required | Description |");
    lines.push("|------|----|------|----------|-------------|");
    for (const p of action.parameters) {
      lines.push(
        `| ${p.name} | ${p.in} | ${p.type} | ${p.required ? "yes" : "no"} | ${p.description} |`,
      );
    }
  }

  if (action.requestBody) {
    lines.push("", "## Request Body", "");
    lines.push(`Content-Type: ${action.requestBody.contentType}`, "");
    lines.push("```json");
    lines.push(JSON.stringify(action.requestBody.schema, null, 2));
    lines.push("```");
  }

  if (action.responses.length > 0) {
    lines.push("", "## Responses", "");
    for (const r of action.responses) {
      lines.push(`### ${r.status} — ${r.description}`);
      if (r.schema) {
        lines.push("");
        lines.push("```json");
        lines.push(JSON.stringify(r.schema, null, 2));
        lines.push("```");
      }
      lines.push("");
    }
  }

  return lines.join("\n");
}
