const API_BASE_URL = "http://localhost:3000"; // バックエンドのURL

// JWT トークンを localStorage に保存
export const setAuthToken = (token: string | null) => {
  if (token) {
    localStorage.setItem("token", token);
  } else {
    localStorage.removeItem("token");
  }
};

// fetch のラッパー関数
const fetchAPI = async (url: string, options: RequestInit = {}) => {
  const token = localStorage.getItem("token");
  const headers: HeadersInit = {
    "Content-Type": "application/json",
    ...(token ? { Authorization: `Bearer ${token}` } : { Authorization: `` }),
  };

  const response = await fetch(`${API_BASE_URL}${url}`, {
    ...options,
    credentials: "include",
    headers,
  });

  if (!response.ok) {
    const errorMessage = await response.text();
    throw new Error(errorMessage || "Request failed");
  }

  return response.json();
};

// API 関数
export const api = {
  login: (credentials: { username: string; password: string }) =>
    fetchAPI("/login", {
      method: "POST",
      body: JSON.stringify(credentials),
    }),

  register: (userInfo: { username: string; password: string }) =>
    fetchAPI("/register", {
      method: "POST",
      body: JSON.stringify(userInfo),
    }),

  logout: () =>
    fetchAPI("/logout", {
      method: "POST",
    }),

  getUser: () => fetchAPI("/me"),
};
