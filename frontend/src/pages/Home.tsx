import React, { useContext } from "react";
import { Container, Title, Button, Box, Flex } from "@mantine/core";
import { IconPlus } from "@tabler/icons-react";
import { Link, useNavigate } from "react-router-dom";
import { useDisclosure } from "@mantine/hooks";
import CreateChatModal from "../components/organisms/ChatCreateModal";
import { useAuth } from "../hooks/useAuth";
import { AuthContext } from "../layout/AuthProvider";

const Home: React.FC = () => {
  const [opened, { open, close }] = useDisclosure(false);
  const auth = useContext(AuthContext);
  const navigate = useNavigate();

  if (auth == null) {
    navigate("/login");
    return;
  }

  return (
    <Container size="md" p="md">
      <CreateChatModal
        isOpen={opened}
        onClose={close}
        onChatCreated={close}
        auth={auth}
      />
      <Flex align="center" justify="center" direction="column" gap="xl">
        <Box>
          <Title order={2}>お帰りなさい</Title>
        </Box>
        <Box>
          <Button
            leftSection={<IconPlus size={18} />}
            variant="light"
            onClick={open}
          >
            新しいチャットを作成
          </Button>
        </Box>
      </Flex>
    </Container>
  );
};

export default Home;
