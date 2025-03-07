from collections import deque


def person_is_seller(name):
    return name[-1] == 'm'


def search(graph, name):
    search_queue = deque()
    search_queue.append(name)
    print(search_queue)
    searched = set()

    while search_queue:
        person = search_queue.popleft()
        if person not in searched:
            if person_is_seller(person):
                print(f"{person} is a mango seller!")
                return True
            else:
                for idx in graph[person]:
                    search_queue.append(idx)
                searched.add(person)
    return False


graph = {
    "you": ["alice", "bob", "claire"],
    "bob": ["anuj", "peggy"],
    "alice": ["peggy"],
    "claire": ["thom", "jonny"],
    "anuj": [],
    "peggy": [],
    "thom": [],
    "jonny": []
}


search(graph, "you")
