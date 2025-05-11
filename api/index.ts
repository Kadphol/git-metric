import { serve } from "bun";
import { type Metric } from "./types";
import { config } from "dotenv";
config();

const TOKEN = process.env.GITLAB_TOKEN!;
const PROJECT_ID = process.env.GITLAB_PROJECT_ID!;
const BASE_URL = process.env.GITLAB_BASE_URL!;

async function fetchGitLabAPI<T>(path: string): Promise<T> {
  const res = await fetch(`${BASE_URL}/api/v4${path}`, {
    headers: { Authorization: `Bearer ${TOKEN}` },
  });
  return res.json();
}

async function getMetrics(): Promise<Metric[]> {
  const mrs = await fetchGitLabAPI<any[]>(
    `/projects/${PROJECT_ID}/merge_requests?state=merged&per_page=30`,
  );
  const metrics: Metric[] = [];

  for (const mr of mrs) {
    const createdAt = new Date(mr.created_at);
    const title = mr.title;
    const author = mr.author.username;
    const iid = mr.iid;

    const commits = await fetchGitLabAPI<any[]>(
      `/projects/${PROJECT_ID}/merge_requests/${iid}/commits`,
    );
    const firstCommitAt = new Date(commits[0]?.created_at ?? mr.created_at);
    const codingTime =
      (createdAt.getTime() - firstCommitAt.getTime()) / 3600000;

    const notes = await fetchGitLabAPI<any[]>(
      `/projects/${PROJECT_ID}/merge_requests/${iid}/notes`,
    );
    const responseNote = notes.find((note) => note.author.username !== author);
    const responseTime = responseNote
      ? (new Date(responseNote.created_at).getTime() - createdAt.getTime()) /
        3600000
      : null;

    metrics.push({
      title,
      author,
      coding_time_hrs: Math.round(codingTime * 100) / 100,
      pr_response_time_hrs: responseTime
        ? Math.round(responseTime * 100) / 100
        : null,
    });
  }

  return metrics;
}

const CORS_HEADERS = {
  headers: {
    "Access-Control-Allow-Origin": "*",
    "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE, OPTIONS",
    "Access-Control-Allow-Headers": "Content-Type, Authorization",
  },
};

serve({
  port: 8080,
  idleTimeout: 60,
  routes: {
    "/api/metrics": {
      GET: async () => {
        const metrics = await getMetrics();
        return Response.json(metrics, CORS_HEADERS);
      },
    },
    "/api/*": Response.json({ message: "Not found" }, { status: 404 }),
  },
});
