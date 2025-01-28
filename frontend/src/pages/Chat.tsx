import React from "react";
import { Container, Title } from "@mantine/core";
import { useParams } from "react-router-dom";

const Chat: React.FC = () => {
  const { id } = useParams();

  return (
    <Container size="md" p="md">
      <Title order={2}>チャットルーム: {id}</Title>
      <p>ここにチャットのメッセージを表示</p>
    </Container>
  );
};

export default Chat;
