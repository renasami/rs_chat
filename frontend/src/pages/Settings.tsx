import { IconCheck } from "@tabler/icons-react";
import React, { useState } from "react";
import {
  Container,
  TextInput,
  Button,
  Title,
  Stack,
  Group,
  Paper,
} from "@mantine/core";
import { notifications } from "@mantine/notifications";

const Settings: React.FC = () => {
  const [userId, setUserId] = useState("");
  const [userName, setUserName] = useState("");
  const [error, setError] = useState("");

  const handleSave = () => {
    if (!userId.trim()) {
      setError("ユーザーIDは必須です");
      return;
    }

    console.log("User Settings Saved:", { userId, userName });
    setError("");

    notifications.show({
      title: "設定が保存されました",
      message: `ユーザーID: ${userId}, ユーザー名: ${userName || "未設定"}`,
      color: "green",
      icon: <IconCheck size={18} />,
      autoClose: 3000,
      position: "bottom-left",
    });
  };

  return (
    <Container
      size="sm"
      style={{
        minHeight: "calc(100vh - 60px)", // 🔥 ヘッダーの高さを考慮
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
      }}
    >
      <Paper withBorder shadow="md" p="lg" radius="md" style={{ width: 400 }}>
        <Stack>
          <Title order={2} ta="center">
            ユーザー設定
          </Title>

          <TextInput
            label="ユーザーID"
            placeholder="your_id_123"
            value={userId}
            onChange={(e) => setUserId(e.currentTarget.value)}
            error={error}
            required
          />

          <TextInput
            label="ユーザー名"
            placeholder="あなたの名前"
            value={userName}
            onChange={(e) => setUserName(e.currentTarget.value)}
            required
          />

          <Group p="center">
            <Button
              onClick={handleSave}
              variant="gradient"
              gradient={{ from: "blue", to: "cyan" }}
            >
              保存
            </Button>
          </Group>
        </Stack>
      </Paper>
    </Container>
  );
};

export default Settings;
