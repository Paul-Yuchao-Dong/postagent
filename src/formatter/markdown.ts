import type {
  ProjectInfo,
  ResourceInfo,
  ActionInfo,
} from "../loader/index.js";

export function formatProjectList(projects: ProjectInfo[]): string {
  return projects
    .map((p) => {
      const resourceNames = p.resources.map((r) => r.name).join(", ");
      return `${p.name}\n  ${p.description.trim()}\n  Resources: ${resourceNames}`;
    })
    .join("\n\n");
}

export function formatProject(project: ProjectInfo): string {
  const lines = [
    `${project.name}`,
    `  ${project.description.trim()}`,
    "",
    "Resources:",
  ];
  for (const r of project.resources) {
    const actionNames = r.actions.map((a) => a.name).join(", ");
    lines.push(`  ${r.name}  (${actionNames})`);
  }
  return lines.join("\n");
}

export function formatResource(project: ProjectInfo, resource: ResourceInfo): string {
  const lines = [`${project.name} > ${resource.name}`, "", "Actions:"];
  for (const a of resource.actions) {
    lines.push(`  ${a.name}  ${a.method} ${a.path}  ${a.summary}`);
  }
  return lines.join("\n");
}

export function formatAction(
  project: ProjectInfo,
  resource: ResourceInfo,
  action: ActionInfo,
): string {
  const lines = [
    `${project.name} > ${resource.name} > ${action.name}`,
    "",
    `  ${action.method} ${action.path}`,
    "",
    `  ${action.description.trim()}`,
  ];

  if (action.parameters.length > 0) {
    lines.push("", "Parameters:");
    for (const p of action.parameters) {
      const req = p.required ? "required" : "optional";
      lines.push(`  --${p.name}  <${p.type}>  (${p.in}, ${req})  ${p.description}`);
    }
  }

  if (action.requestBody) {
    lines.push("", `Request Body (${action.requestBody.contentType}):`);
    lines.push(JSON.stringify(action.requestBody.schema, null, 2));
  }

  if (action.responses.length > 0) {
    lines.push("", "Responses:");
    for (const r of action.responses) {
      lines.push(`  ${r.status}  ${r.description}`);
    }
  }

  return lines.join("\n");
}
