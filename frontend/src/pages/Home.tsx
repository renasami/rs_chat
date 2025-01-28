import React from "react";
import { Container, Title, Stack, Button, Box, Flex } from "@mantine/core";
import { IconPlus } from "@tabler/icons-react";
import { Link } from "react-router-dom";

const Home: React.FC = () => {
  return (
    <Container size="md" p="md">
      <Flex align="center" justify="center" direction="column" gap="xl">
        <Box>
          <Title order={2}>お帰りなさい</Title>
        </Box>
        <Box>
          <Button
            component={Link}
            to="/chat/new"
            leftSection={<IconPlus size={18} />}
            variant="light"
          >
            新しいチャットを作成
          </Button>
        </Box>
      </Flex>
    </Container>
  );
};

export default Home;
