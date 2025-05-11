import { createFileRoute } from "@tanstack/react-router";
import { useState, useEffect } from "react";
import axios from "axios";
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  Tooltip,
  CartesianGrid,
  ResponsiveContainer,
} from "recharts";

export const Route = createFileRoute("/")({
  component: App,
});

type Metric = {
  author: string;
  coding_time_hrs: number;
  pr_response_time_hrs?: number;
};

function App() {
  const [data, setData] = useState<Metric[]>([]);

  useEffect(() => {
    axios
      .get<Metric[]>("http://localhost:8080/api/metrics")
      .then((res) => setData(res.data));
  }, []);

  const authors = Array.from(new Set(data.map((d) => d.author)));

  const avg = (values: number[]) =>
    values.reduce((a, b) => a + b, 0) / values.length || 0;

  const aggregated = authors.map((author) => {
    const userData = data.filter((d) => d.author === author);
    return {
      author,
      coding: Number(avg(userData.map((d) => d.coding_time_hrs)).toFixed(2)),
      response: Number(
        avg(userData.map((d) => d.pr_response_time_hrs || 0)).toFixed(2),
      ),
    };
  });

  return (
    <div style={{ padding: "2rem" }}>
      <h2>GitLab Engineering Metrics</h2>
      <ResponsiveContainer width="100%" height={400}>
        <BarChart data={aggregated}>
          <CartesianGrid strokeDasharray="3 3" />
          <XAxis dataKey="author" />
          <YAxis />
          <Tooltip />
          <Bar dataKey="coding" fill="#8884d8" name="Coding Time (hrs)" />
          <Bar
            dataKey="response"
            fill="#82ca9d"
            name="PR Response Time (hrs)"
          />
        </BarChart>
      </ResponsiveContainer>
    </div>
  );
}
