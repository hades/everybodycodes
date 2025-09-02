#include "ec/include/event2/quest1.h"

#include <cstdint>
#include <iostream>
#include <map>
#include <ranges>
#include <string>
#include <vector>

::rust::String solve_part_1(::rust::Str input) {
    std::vector<std::string> nails;
    std::vector<std::string> instructions;
    bool is_instruction = false;
    for (const auto line: ((std::string_view)input) | std::views::split('\n')) {
        if (line.size() == 0) {
            is_instruction = true;
            continue;
        }
        if (is_instruction) {
            instructions.push_back(std::string(std::string_view(line)));
        } else {
            nails.push_back(std::string(std::string_view(line)));
        }
    }
    int total_winnings = 0;
    for (int i = 0; i < 9; ++i) {
        int x = i * 2;
        long unsigned int y = 0;
        int ip = 0;
        while (y < nails.size()) {
            if (nails[y][x] == '*') {
                char instruction = instructions[i][ip];
                ip = (ip + 1) % instructions[i].size();
                x = (instruction == 'L') ? x - 1 : x + 1;
                if (x < 0) x = 1;
                if (x == (int)nails[y].size()) x = nails[y].size() - 2;
            }
            y++;
        }
        int slot = x / 2 + 1;
        total_winnings += std::max(0, slot * 2 - (i + 1));
    }
    return std::to_string(total_winnings);
}

::rust::String solve_part_2(::rust::Str input) {
    std::vector<std::string> nails;
    std::vector<std::string> instructions;
    bool is_instruction = false;
    for (const auto line: ((std::string_view)input) | std::views::split('\n')) {
        if (line.size() == 0) {
            is_instruction = true;
            continue;
        }
        if (is_instruction) {
            instructions.push_back(std::string(std::string_view(line)));
        } else {
            nails.push_back(std::string(std::string_view(line)));
        }
    }
    int total_winnings = 0;
    for (int i = 0; i < instructions.size(); ++i) {
        int max_winnings = 0;
        for (int start_slot = 1; start_slot <= 13; start_slot++) {
            int x = (start_slot - 1) * 2;
            size_t y = 0;
            int ip = 0;
            while (y < nails.size()) {
                if (nails[y][x] == '*') {
                    char instruction = instructions[i][ip];
                    ip = (ip + 1) % instructions[i].size();
                    x = (instruction == 'L') ? x - 1 : x + 1;
                    if (x < 0) x = 1;
                    if (x == (int)nails[y].size()) x = nails[y].size() - 2;
                }
                y++;
            }
            int slot = x / 2 + 1;
            max_winnings = std::max(max_winnings, slot * 2 - start_slot);
            std::cout << "coin " << i << " start_slot " << start_slot << " slot " << slot << " max_winnings " << max_winnings << std::endl;
        }
        total_winnings += max_winnings;
    }
    return std::to_string(total_winnings);
}

::rust::String solve_part_3(::rust::Str input) {
    std::vector<std::string> nails;
    std::vector<std::string> instructions;
    bool is_instruction = false;
    for (const auto line: ((std::string_view)input) | std::views::split('\n')) {
        if (line.size() == 0) {
            is_instruction = true;
            continue;
        }
        if (is_instruction) {
            instructions.push_back(std::string(std::string_view(line)));
        } else {
            nails.push_back(std::string(std::string_view(line)));
        }
    }
    int total_slots = (nails[0].size() + 1) / 2;
    std::map<std::tuple<int, int>, int> winning_per_token_per_slot;
    for (int i = 0; i < instructions.size(); ++i) {
        for (int start_slot = 1; start_slot <= total_slots; start_slot++) {
            int x = (start_slot - 1) * 2;
            size_t y = 0;
            int ip = 0;
            while (y < nails.size()) {
                if (nails[y][x] == '*') {
                    char instruction = instructions[i][ip];
                    ip = (ip + 1) % instructions[i].size();
                    x = (instruction == 'L') ? x - 1 : x + 1;
                    if (x < 0) x = 1;
                    if (x == (int)nails[y].size()) x = nails[y].size() - 2;
                }
                y++;
            }
            int slot = x / 2 + 1;
            winning_per_token_per_slot[{i, start_slot}] = std::max(0, slot * 2 - start_slot);
        }
    }
    int max_winnings = 0;
    int min_winnings = std::numeric_limits<int>::max();
    for (int a = 0; a < total_slots; ++a) {
        for (int b = 0; b < total_slots; ++b) {
            if (b == a) continue;
            for (int c = 0; c < total_slots; ++c) {
                if (c == a || c == b) continue;
                for (int d = 0; d < total_slots; ++d) {
                    if (d == a || d == b || d == c) continue;
                    for (int e = 0; e < total_slots; ++e) {
                        if (e == a || e == b || e == c || e == d) continue;
                        for (int f = 0; f < total_slots; ++f) {
                            if (f == a || f == b || f == c || f == d || f == e) continue;
                            int winning = winning_per_token_per_slot[{0, a + 1}]
                            + winning_per_token_per_slot[{1, b + 1}]
                            + winning_per_token_per_slot[{2, c + 1}]
                            + winning_per_token_per_slot[{3, d + 1}]
                            + winning_per_token_per_slot[{4, e + 1}]
                            + winning_per_token_per_slot[{5, f + 1}];
                            max_winnings = std::max(max_winnings, winning);
                            min_winnings = std::min(min_winnings, winning);
                        }
                    }
                }
            }
        }
    }
    return std::to_string(min_winnings) + " " + std::to_string(max_winnings);
}