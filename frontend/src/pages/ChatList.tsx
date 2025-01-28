import React from "react";
import {
  Card,
  Text,
  Group,
  Stack,
  Avatar,
  Container,
  Title,
} from "@mantine/core";
import { Link } from "react-router-dom";

// サンプルデータ（実際はAPIなどから取得）
const chatRooms = [
  {
    id: "1",
    name: "General Chat",
    lastMessage: "Hey everyone!",
    lastMessageTime: "15:30",
  },
  {
    id: "2",
    name: "Project Discussion",
    lastMessage: "Let's finalize the report.",
    lastMessageTime: "14:20",
  },
  {
    id: "3",
    name: "Random Talks",
    lastMessage: "Anyone up for a coffee?",
    lastMessageTime: "13:05",
  },
];

const ChatList: React.FC = () => {
  return (
    <Container size="md" p="md">
      <Stack>
        <Title order={2}>チャット一覧</Title>
        {chatRooms.map((room) => (
          <Card
            key={room.id}
            component={Link}
            to={`/chat/${room.id}`}
            shadow="sm"
            padding="lg"
            radius="md"
            withBorder
            style={{ textDecoration: "none" }}
          >
            <Group>
              <Avatar color="blue" radius="xl">
                {room.name.charAt(0)}
              </Avatar>
              <div>
                <Text w={500}>{room.name}</Text>
                <Text size="sm" color="dimmed">
                  {room.lastMessage}
                </Text>
                <Text size="xs" color="dimmed">
                  {room.lastMessageTime}
                </Text>
              </div>
            </Group>
          </Card>
        ))}
      </Stack>
    </Container>
  );
};

export default ChatList;
