import { useCallback, useEffect, useState } from "react";
import { api } from "../../../lib/api";
import type { ProcessSession } from "../types";

export function useSessions() {
  const [sessions, setSessions] = useState<ProcessSession[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [lastUpdated, setLastUpdated] = useState<Date | null>(null);

  const refresh = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await api.listSessions();
      setSessions(result);
      setLastUpdated(new Date());
    } catch (cause) {
      setError(String(cause));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => { void refresh(); }, [refresh]);

  return { sessions, error, loading, lastUpdated, refresh };
}
