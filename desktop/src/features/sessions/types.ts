export type ProcessSession = {
  id: string;
  pid: number;
  name: string;
  command: string;
  start_time_seconds: number;
  start_time_useconds: number;
};

export type NewSessionInput = {
  name: string;
  program: string;
  args: string[];
};
