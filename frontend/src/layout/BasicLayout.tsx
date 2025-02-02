import { Link, Navigate } from "react-router-dom";
import { AppShell, Group, Button, Box } from "@mantine/core";
import {
  IconHome,
  IconSettings,
  IconBrandRust,
  IconLogout,
  IconLayoutList,
} from "@tabler/icons-react";
import { Outlet } from "react-router-dom";
import { JSX, useContext, useEffect } from "react";
import { AuthContext } from "./AuthProvider";

export const BasicLayout = () => {
  const handleLogout = () => {
    console.log("Logging out...");
    // ここにログアウト処理を追加
  };
  const data = useContext(AuthContext);

  if (data === null) return <Navigate to="/login" />;
  if (data.isLoading) return <p>Loading...</p>;
  if (data.user === null) return <Navigate to="/login" />;

  return (
    <AppShell
      header={{ height: 60 }}
      navbar={{
        width: 300,
        breakpoint: "sm",
      }}
      aside={{ width: 0, breakpoint: "xs" }}
      padding="md"
    >
      {/* ヘッダー */}
      <AppShell.Header p="md">
        <Group justify="space-between">
          <Group>
            <IconBrandRust size={30} />
            <h1 style={{ margin: 0, fontSize: "18px" }}>My Rust Chat App</h1>
          </Group>

          <Button
            variant="light"
            color="red"
            leftSection={<IconLogout size={18} />}
            onClick={handleLogout}
          >
            ログアウト
          </Button>
        </Group>
      </AppShell.Header>

      {/* サイドバー */}
      <AppShell.Navbar p="md">
        <Box>
          <Button
            component={Link}
            to="/"
            fullWidth
            leftSection={<IconHome size={18} />}
          >
            ホーム
          </Button>
        </Box>
        <Box mt="md">
          <Button
            component={Link}
            to="/chats"
            fullWidth
            leftSection={<IconLayoutList size={18} />}
          >
            チャット一覧
          </Button>
        </Box>
        <Box mt="md">
          <Button
            component={Link}
            to="/settings"
            fullWidth
            leftSection={<IconSettings size={18} />}
          >
            設定
          </Button>
        </Box>
      </AppShell.Navbar>
      <AppShell.Main>
        <Outlet />
      </AppShell.Main>
    </AppShell>
  );
};
