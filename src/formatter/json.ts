import type {
  ProjectInfo,
  ResourceInfo,
  ActionInfo,
} from "../loader/index.js";

export function formatProjectList(projects: ProjectInfo[]): string {
  return JSON.stringify(
    projects.map((p) => ({
      name: p.name,
      description: p.description.trim(),
      resources: p.resources.map((r) => r.name),
    })),
    null,
    2,
  );
}

export function formatProject(project: ProjectInfo): string {
  return JSON.stringify(
    {
      name: project.name,
      description: project.description.trim(),
      resources: project.resources.map((r) => ({
        name: r.name,
        actions: r.actions.map((a) => a.name),
      })),
    },
    null,
    2,
  );
}

export function formatResource(_project: ProjectInfo, resource: ResourceInfo): string {
  return JSON.stringify(
    {
      resource: resource.name,
      actions: resource.actions.map((a) => ({
        name: a.name,
        method: a.method,
        path: a.path,
        summary: a.summary,
      })),
    },
    null,
    2,
  );
}

export function formatAction(
  project: ProjectInfo,
  resource: ResourceInfo,
  action: ActionInfo,
): string {
  return JSON.stringify(
    {
      project: project.name,
      resource: resource.name,
      action: action.name,
      method: action.method,
      path: action.path,
      description: action.description.trim(),
      parameters: action.parameters,
      requestBody: action.requestBody ?? null,
      responses: action.responses,
    },
    null,
    2,
  );
}
